//! #5821: 장평은 가로에만 적용하고 제목의 선언 크기와 정렬 점유를 보존한다.
//!
//! 같은 원본의 독립 Print PDF와 전쪽 Native/fresh WASM 선행 검증은
//! mydocs/pr/assets/issue7445/resolution-20261005/
//! regression5821-terminal-tracking-fresh-validation.json에 있다.
//! PDF의 가로·세로 행렬을 확인해 예전 √r 세로 축소 기대값을 교정했다.
#![cfg(not(target_arch = "wasm32"))]

use rhwp::model::control::Control;
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};
use rhwp::DocumentCore;

const SAMPLE: &str = "samples/issue5821/condensed_ratio_title_box.hwpx";

fn core() -> DocumentCore {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(SAMPLE);
    DocumentCore::from_bytes(&std::fs::read(path).expect("정식 원본 읽기")).expect("문서 파싱")
}

fn title(core: &DocumentCore) -> &rhwp::model::paragraph::Paragraph {
    let table = core.document().sections[0]
        .paragraphs
        .iter()
        .find_map(|para| {
            para.controls.iter().find_map(|control| match control {
                Control::Table(table) => Some(table),
                _ => None,
            })
        })
        .expect("제목 표");
    let para = &table.cells[0].paragraphs[0];
    assert!(para.text.starts_with("경찰청,"), "원본 제목 소속");
    para
}

struct Glyph {
    text: String,
    x: f64,
    y: f64,
    size: f64,
    ratio: f64,
}

fn title_glyphs(svg: &str) -> Vec<Glyph> {
    let xml = roxmltree::Document::parse(svg).expect("실제 SVG");
    xml.descendants()
        .filter(|node| {
            node.has_tag_name("text")
                && node
                    .attribute("font-family")
                    .is_some_and(|family| family.starts_with("'함초롬돋움'"))
                && node.attribute("font-weight") == Some("bold")
        })
        .map(|node| {
            let transform = node.attribute("transform").expect("장평 변환");
            let translate = transform
                .strip_prefix("translate(")
                .expect("글자 원점")
                .split(')')
                .next()
                .unwrap();
            let mut xy = translate
                .split(',')
                .map(|value| value.parse::<f64>().unwrap());
            let scale = transform
                .split("scale(")
                .nth(1)
                .expect("가로 장평")
                .split(',')
                .next()
                .unwrap();
            Glyph {
                text: node.text().expect("제목 글자").to_owned(),
                x: xy.next().unwrap(),
                y: xy.next().unwrap(),
                size: node.attribute("font-size").unwrap().parse().unwrap(),
                ratio: scale.parse().unwrap(),
            }
        })
        .collect()
}

#[test]
fn issue_5821_title_keeps_declared_glyph_size_and_content() {
    let core = core();
    assert_eq!(core.page_count(), 1, "독립 PDF와 원본의 쪽수");
    let para = title(&core);
    let shapes: Vec<_> = para
        .char_shapes
        .iter()
        .map(|reference| &core.document().doc_info.char_shapes[reference.char_shape_id as usize])
        .collect();
    let declared_size = f64::from(shapes[0].base_size) * 96.0 / 7200.0;
    let declared_ratio = f64::from(shapes[0].ratios[0]) / 100.0;
    assert!(shapes
        .iter()
        .all(|shape| shape.base_size == shapes[0].base_size
            && shape.ratios[0] == shapes[0].ratios[0]));
    let svg = core.render_page_svg_native(0).expect("실제 제목 렌더링");
    let glyphs = title_glyphs(&svg);
    let actual: String = glyphs.iter().map(|glyph| glyph.text.as_str()).collect();
    let expected: String = para.text.chars().filter(|ch| !ch.is_whitespace()).collect();
    assert_eq!(actual, expected, "두 제목 줄의 내용·순서·누락·중복");
    for glyph in glyphs {
        assert!(
            (glyph.size - declared_size).abs() <= declared_size * f64::EPSILON * 64.0,
            "장평이 선언 세로 크기를 바꾸면 안 된다: {} / 선언 {}",
            glyph.size,
            declared_size
        );
        assert_eq!(glyph.ratio, declared_ratio, "장평은 원래 가로 비율이다");
    }
}

fn title_cell(node: &RenderNode) -> Option<&RenderNode> {
    if matches!(&node.node_type, RenderNodeType::TableCell(cell) if cell.row == 0 && cell.col == 0)
    {
        return Some(node);
    }
    node.children.iter().find_map(title_cell)
}

#[test]
fn issue_5821_first_title_visible_span_is_centered_in_its_cell() {
    let core = core();
    let svg = core.render_page_svg_native(0).expect("실제 제목 SVG");
    let glyphs = title_glyphs(&svg);
    let first_y = glyphs
        .iter()
        .map(|glyph| glyph.y)
        .min_by(f64::total_cmp)
        .expect("첫 제목 줄");
    let first_line: Vec<_> = glyphs.iter().filter(|glyph| glyph.y == first_y).collect();
    let first = first_line.first().unwrap();
    let last = first_line.last().unwrap();
    assert_eq!(last.text, "여", "독립 PDF와 같은 첫 제목 줄 종료");
    // 독립 PDF의 마지막 한글 advance와 실제 공급 프로그램은 970/1000이다.
    // 마지막 글자 뒤에는 다음 글자가 없으므로 자간을 뺀 전진이 아닌 자연 폭을 쓴다.
    let visible_end = last.x + last.size * last.ratio * 970.0 / 1000.0;
    let tree = core.build_page_render_tree(0).expect("제목 셀의 실제 상자");
    let cell = title_cell(&tree.root).expect("제목 셀 소속");
    let span_center = (first.x + visible_end) / 2.0;
    let cell_center = cell.bbox.x + cell.bbox.width / 2.0;
    // HU 한 단위의 변환 정밀도만 허용한다. 실제 문서의 절대 좌표를 고정하지 않는다.
    assert!(
        (span_center - cell_center).abs() <= 96.0 / 7200.0,
        "첫 제목 가시 폭이 셀 중심에 맞아야 한다: 제목 {span_center}, 셀 {cell_center}"
    );
}
