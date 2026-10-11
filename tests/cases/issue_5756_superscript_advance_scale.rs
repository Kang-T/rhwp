//! #5756: 위첨자 줄의 원본 내용과 셀 내부 전진 범위를 보존한다.
//!
//! 같은 원본의 사용자 수동 한컴 Print 3쪽 PDF와 전쪽 Native/fresh WASM 증적은
//! mydocs/pr/assets/issue7445/completed-20261009/stored-anchor-repair/validation.json
//! 의 superscript_manual_print 항목에 있다. 장평의 세로 축소와 절대 좌표를 고정하지 않는다.
#![cfg(not(target_arch = "wasm32"))]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use rhwp::document_core::DocumentCore;
use rhwp::model::control::Control;
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};

type Owner = (usize, usize, usize, usize, usize);

fn compact(text: &str) -> String {
    text.chars()
        .filter(|ch| !ch.is_whitespace() && !ch.is_control())
        .collect()
}

fn cell_runs<'a>(
    node: &'a RenderNode,
    cell: Option<&'a RenderNode>,
    out: &mut Vec<(&'a RenderNode, &'a RenderNode)>,
) {
    let cell = if matches!(node.node_type, RenderNodeType::TableCell(_)) {
        Some(node)
    } else {
        cell
    };
    if let RenderNodeType::TextRun(run) = &node.node_type {
        if let Some(cell) = cell.filter(|_| !run.display_or_text().trim().is_empty()) {
            out.push((node, cell));
        }
    }
    for child in &node.children {
        cell_runs(child, cell, out);
    }
}

#[test]
fn issue_5756_superscript_line_stays_inside_cell() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(SAMPLE);
    let core = DocumentCore::from_bytes(&std::fs::read(path).expect("원본 읽기")).expect("열기");
    assert_eq!(core.page_count(), 3, "독립 한컴 Print와 같은 전체 쪽수");
    let tree = core.build_page_render_tree(2).expect("위첨자 표의 3쪽");
    let svg = core.render_page_svg_native(2).expect("3쪽 SVG");
    let xml = roxmltree::Document::parse(&svg).expect("실제 그리기 출력");
    let sizes: Vec<f64> = xml
        .descendants()
        .filter(|node| node.has_tag_name("text"))
        .map(|node| {
            node.attribute("font-size")
                .expect("글리프 크기")
                .parse()
                .unwrap()
        })
        .collect();
    let mut runs = Vec::new();
    // 쪽 번호·머리말은 표의 검사 범위가 아니다.
    cell_runs(&tree.root, None, &mut runs);
    let mut text_by_owner = BTreeMap::<Owner, String>::new();
    let mut super_owners = BTreeSet::new();
    let mut ordinary = 0;
    for (node, cell) in runs {
        let RenderNodeType::TextRun(run) = &node.node_type else {
            unreachable!()
        };
        let shape = &core.document().doc_info.char_shapes
            [run.char_shape_id.expect("원본 글자모양") as usize];
        let context = run.cell_context.as_ref().expect("원본 표 소속");
        assert_eq!(context.path.len(), 1, "검사 대상은 원본의 단일 표 셀");
        let entry = context.path[0];
        let owner = (
            run.section_index.unwrap(),
            context.parent_para_index,
            entry.control_index,
            entry.cell_index,
            entry.cell_para_index,
        );
        text_by_owner
            .entry(owner)
            .or_default()
            .push_str(run.display_or_text());
        // 실제 run 전체 전진이 셀 내부에 남아야 한다. 시작 x만 검사하지 않는다.
        let precision = run.style.font_size * f64::EPSILON * 128.0;
        assert!(
            node.bbox.x >= cell.bbox.x - precision
                && node.bbox.x + node.bbox.width <= cell.bbox.x + cell.bbox.width + precision,
            "글줄 전진은 소유 셀 안에 있어야 한다: {:?}, {:?}, {:?}",
            run.text,
            node.bbox,
            cell.bbox
        );
        if shape.superscript {
            super_owners.insert(owner);
            assert!(
                shape.relative_sizes.iter().all(|size| *size == 100),
                "원본의 동일 상대 크기"
            );
            let base = f64::from(shape.base_size) * 96.0 / 7200.0;
            // 기존 위첨자 70% 그리기 계약을 원본 기준 크기에 적용한다. 장평은 가로만 바꾼다.
            for expected in [base, base * 0.7] {
                assert!(
                    sizes
                        .iter()
                        .any(|size| (size - expected).abs() <= base * f64::EPSILON * 128.0),
                    "원본 기준 크기와 위첨자 그리기 배율을 보존한다: {expected}"
                );
            }
        } else {
            ordinary += 1;
        }
    }
    assert!(
        !super_owners.is_empty() && ordinary > 0,
        "위첨자와 본문이 함께 있어야 한다"
    );
    for (section, para, control, cell, paragraph) in super_owners {
        let Control::Table(table) =
            &core.document().sections[section].paragraphs[para].controls[control]
        else {
            panic!("원본 표 소속");
        };
        let source = &table.cells[cell].paragraphs[paragraph].text;
        assert_eq!(
            compact(&text_by_owner[&(section, para, control, cell, paragraph)]),
            compact(source),
            "위첨자와 본문으로 구성된 원래 문단의 내용·순서·누락·중복"
        );
    }
}

const SAMPLE: &str = "samples/issue5756/156732409_superscript_advance.hwp";
