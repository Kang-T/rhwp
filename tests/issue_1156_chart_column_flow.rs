//! Issue #1156: 2단 문서에서 차트(OLE) 컨트롤의 단 이동 + 자리차지 텍스트
//! 후속 본문 채움과 페이지 배경 워터마크 효과 회귀 가드.
//!
//! 재현 문서: `samples/hwpx/143E433F503322BD33.hwpx` (2단 + 차트 OLE + 표 + 배경 워터마크).
//!
//! 검증 항목:
//! 1. 차트(80mm, 비-TAC TopAndBottom)가 단0 끝을 넘어 단1 상단으로 이동.
//! 2. 자리차지(TopAndBottom) 속성이므로 단1 후속 텍스트가 차트 영역과 겹치지 않음
//!    절대 페이지 좌표 대신 소속 단과 차트 뒤 본문 관계를 검사.
//!    같은 문단의 처음 두 줄은 첫 단, 나머지 네 줄은 둘째 단에 보존.
//!    #7445 보정 후 두 형식의 Native/fresh WASM 전쪽 90% 이상 증적은
//!    mydocs/pr/assets/issue7445/resolution-20261005/fonttrace4961-column-flow.json.
//! 3. 페이지 배경 이미지(워터마크)는 반투명 합성 — SVG `<g opacity=...>` 적용
//!    (PR #1019 RealPic 톤 프리셋 사각지대 회귀 가드).

use std::fs;
use std::path::Path;

fn render_page_svg(rel: &str, page: u32) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    let bytes = fs::read(&path).unwrap_or_else(|e| panic!("read {}: {}", rel, e));
    let doc = rhwp::wasm_api::HwpDocument::from_bytes(&bytes).expect("parse");
    doc.render_page_svg(page).expect("render svg")
}

#[test]
fn chart_moves_to_second_column_and_text_does_not_overlap() {
    use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};

    fn find_column(node: &RenderNode, index: u16) -> Option<&RenderNode> {
        if matches!(node.node_type, RenderNodeType::Column(i) if i == index) {
            return Some(node);
        }
        node.children
            .iter()
            .find_map(|child| find_column(child, index))
    }
    fn collect<'a>(
        node: &'a RenderNode,
        charts: &mut Vec<&'a RenderNode>,
        lines: &mut Vec<&'a RenderNode>,
    ) {
        if matches!(&node.node_type, RenderNodeType::RawSvg(svg) if svg.control_ref.as_ref().is_some_and(|r| r.kind == "ole"))
        {
            charts.push(node);
        }
        if matches!(&node.node_type, RenderNodeType::TextLine(_)) {
            lines.push(node);
        }
        for child in &node.children {
            collect(child, charts, lines);
        }
    }
    // 같은 원문의 두 형식 모두 독립 PDF와 Native/fresh WASM 전쪽 비교를 통과했다.
    // 단 번호·원본 문단 소속·비겹침 관계를 검사하고 절대 페이지 좌표를 고정하지 않는다.
    for sample in [
        "samples/143E433F503322BD33.hwp",
        "samples/hwpx/143E433F503322BD33.hwpx",
    ] {
        let bytes =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(sample)).expect("원본 문서");
        let doc = rhwp::DocumentCore::from_bytes(&bytes).expect("원본 로드");
        assert_eq!(doc.page_count(), 1, "독립 PDF의 한 쪽: {sample}");
        let tree = doc.build_page_render_tree(0).expect("첫 쪽 배치");
        let left = find_column(&tree.root, 0).expect("첫 단");
        let right = find_column(&tree.root, 1).expect("둘째 단");
        let (mut left_charts, mut left_lines) = (Vec::new(), Vec::new());
        let (mut right_charts, mut right_lines) = (Vec::new(), Vec::new());
        collect(left, &mut left_charts, &mut left_lines);
        collect(right, &mut right_charts, &mut right_lines);
        assert!(left_charts.is_empty(), "차트만 둘째 단으로 이월: {sample}");
        assert_eq!(right_charts.len(), 1, "차트 누락·중복 금지: {sample}");
        let chart = right_charts[0];
        assert!(
            chart.bbox.y > right.bbox.y
                && chart.bbox.y + chart.bbox.height <= right.bbox.y + right.bbox.height,
            "위 여백을 가진 차트는 소속 단 안에 놓인다: {sample}"
        );
        assert!(
            right_lines
                .iter()
                .all(|line| line.bbox.y >= chart.bbox.y + chart.bbox.height),
            "둘째 단 본문은 차트 뒤에 놓인다: {sample}"
        );
        assert_eq!(left_lines.iter().filter(|line| matches!(&line.node_type, RenderNodeType::TextLine(text) if text.para_index == Some(9))).count(), 2, "호스트 표 뒤 두 줄은 첫 단에 남는다: {sample}");
        assert_eq!(right_lines.iter().filter(|line| matches!(&line.node_type, RenderNodeType::TextLine(text) if text.para_index == Some(9))).count(), 4, "같은 문단의 나머지 네 줄은 둘째 단에 이어진다: {sample}");
    }
}

#[test]
fn page_background_watermark_has_opacity() {
    let svg = render_page_svg("samples/hwpx/143E433F503322BD33.hwpx", 0);
    // 페이지 배경 워터마크는 반투명 합성 — <g opacity="..."> 그룹으로 감쌈.
    // PR #1019(#975) RealPic 톤 프리셋 사각지대로 opacity 가 빠졌던 회귀 가드.
    assert!(
        svg.contains("<g opacity=\""),
        "페이지 배경 워터마크 opacity 그룹이 없음 (워터마크 효과 회귀)"
    );
}
