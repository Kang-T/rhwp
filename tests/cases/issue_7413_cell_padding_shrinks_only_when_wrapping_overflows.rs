//! [#7413] 칸 안 여백 축소는 **줄바꿈으로 해결되지 않을 때만** 발화한다.
//!
//! # 무엇이 깨져 있었나
//!
//! `shrunk_cell_horizontal_padding` 은 칸 문단의 **자연 폭**(줄바꿈하지 않은 한 줄 폭)이
//! 가용 너비의 1.15배를 넘으면 선언된 칸 안 여백을 양쪽 1.0px 까지 깎았다. 그 조건은
//! "이 문단은 줄바꿈이 필요하다"는 뜻일 뿐이라, **줄바꿈하면 칸에 들어가는 문단까지 전부**
//! 걸렸다.
//!
//! 축소가 실제로 지탱하는 계약은 "칸 안 글자가 칸 밖으로 새지 않는다" 하나다. 축소를
//! 통째로 끄고 `samples/**` 전수를 재면 겹침이 4건 늘고, 그 넷이 전부 **줄바꿈 결과가
//! 행 높이를 넘는** 자리였다.
//!
//! 기존 영월군 검사는 `월별누계`를 한 줄로 요구했다. 원본 HWP3의 해당 줄은
//! 시작·좌표·구분 정보가 모두 비어 있는 글자 높이 레코드라 저장 줄 구성의 근거가 아니다.
//! 독립 한컴 2020 Print와 같은 원본의 한컴 HWPX 변환본은 `월별` / `누계` 두 줄이다.
//! 마지막 줄 뒤 간격을 제외한 두 줄은 원본 셀 높이에 들어간다.
//!
//! 영월군 전4쪽 Native/fresh WASM 최저 99.53097% 확인 후 기존 두번째 검사만
//! 원본 셀 소속·두 줄 내용 순서·셀 내부 포함 관계로 교정했다. 다른 입력의 첫번째
//! 여백 계약은 유지한다. 증거는 `stored-anchor-repair/validation.json`의 V14 기록이다.

#![cfg(not(target_arch = "wasm32"))]

use rhwp::document_core::DocumentCore;
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};

/// 줄바꿈이 칸 높이에 들어가는 칸 — 선언 여백이 남아야 한다.
const FITS: &str = "samples/hwpx/table-text.hwpx";
/// 저장 줄 구성이 없는 HWP3 셀 — 독립 Print의 두 줄을 셀 안에 보존한다.
const REFLOWED_HWP3: &str = "samples/issue5699/20099369_yeongwol_forms.hwp";

fn load(rel: &str) -> DocumentCore {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{rel} 읽기: {e}"));
    DocumentCore::from_bytes(&bytes).expect("문서 로드")
}

/// `(칸 bbox, 글자 왼끝, 글자 오른끝)` 를 칸 단위로 모은다.
fn cell_text_extents(
    node: &RenderNode,
    cell: Option<&RenderNode>,
    out: &mut Vec<(f64, f64, f64, f64)>,
) {
    let next_cell = if matches!(node.node_type, RenderNodeType::TableCell(_)) {
        Some(node)
    } else {
        cell
    };
    if let (RenderNodeType::TextRun(_), Some(c)) = (&node.node_type, next_cell) {
        out.push((
            c.bbox.x,
            c.bbox.width,
            node.bbox.x,
            node.bbox.x + node.bbox.width,
        ));
    }
    for child in &node.children {
        cell_text_extents(child, next_cell, out);
    }
}

/// `(칸 x, 칸 y, 칸 폭, 그 칸의 TextLine 수)`.
/// 줄바꿈으로 해결되는 칸은 선언 안 여백을 지킨다.
#[test]
fn a_cell_that_wraps_within_its_height_keeps_its_declared_padding() {
    let core = load(FITS);
    let page = core.build_page_render_tree(0).expect("1쪽 render tree");
    let mut rows = Vec::new();
    cell_text_extents(&page.root, None, &mut rows);

    // 대상 칸: 숫자 한 줄이 들어가는 폭 76.4px 칸.
    let target: Vec<_> = rows
        .iter()
        .filter(|(cx, cw, ..)| (*cx - 115.30).abs() < 0.2 && (*cw - 76.40).abs() < 0.2)
        .collect();
    assert!(
        !target.is_empty(),
        "정답지 전제가 깨졌다 — 대상 칸(x=115.30, w=76.40)을 못 찾았다. 쪽 구성이 바뀌었으면          이 검사의 좌표부터 다시 정해야 한다."
    );

    let left = target
        .iter()
        .map(|(_, _, x0, _)| *x0)
        .fold(f64::MAX, f64::min);
    let right = target
        .iter()
        .map(|(_, _, _, x1)| *x1)
        .fold(f64::MIN, f64::max);
    let (cx, cw) = (target[0].0, target[0].1);
    let pad_left = left - cx;
    let pad_right = cx + cw - right;

    assert!(
        pad_left > 3.0 && pad_right > 3.0,
        "줄바꿈이 칸 높이에 들어가는데도 안 여백이 깎였다 — 축소 하한(1.0px)까지 내려가면          여기서 1.0 근처가 나온다. 좌 {pad_left:.2}px / 우 {pad_right:.2}px"
    );
}

/// 원본 셀과 독립 Print의 내용 구성을 보호하고 과거 페이지/y 좌표를 선택자로 쓰지 않는다.
#[test]
fn b_uncomputed_hwp3_cell_preserves_the_oracle_wrapped_content() {
    use rhwp::model::control::Control;

    fn text(node: &RenderNode) -> String {
        if !node.visible || node.editor_only {
            return String::new();
        }
        let mut s = match &node.node_type {
            RenderNodeType::TextRun(run) => run.display_or_text().to_owned(),
            _ => String::new(),
        };
        for child in &node.children {
            s.push_str(&text(child));
        }
        s
    }
    fn locate<'a>(
        node: &'a RenderNode,
        owner: (usize, usize, usize),
        in_owner: bool,
        cell_index: usize,
        found: &mut Vec<&'a RenderNode>,
    ) {
        if !node.visible || node.editor_only {
            return;
        }
        let in_owner = match &node.node_type {
            RenderNodeType::Table(t) => {
                (t.section_index, t.para_index, t.control_index)
                    == (Some(owner.0), Some(owner.1), Some(owner.2))
                    && t.cell_context.is_none()
            }
            _ => in_owner,
        };
        if let RenderNodeType::TableCell(cell) = &node.node_type {
            if in_owner && cell.model_cell_index == Some(cell_index as u32) {
                found.push(node);
            }
        }
        for child in &node.children {
            locate(child, owner, in_owner, cell_index, found);
        }
    }
    fn lines<'a>(node: &'a RenderNode, found: &mut Vec<&'a RenderNode>) {
        if !node.visible || node.editor_only {
            return;
        }
        if matches!(node.node_type, RenderNodeType::TextLine(_)) && !text(node).trim().is_empty() {
            found.push(node);
        }
        for child in &node.children {
            lines(child, found);
        }
    }

    let core = load(REFLOWED_HWP3);
    let mut source = Vec::new();
    for (si, section) in core.document().sections.iter().enumerate() {
        for (pi, para) in section.paragraphs.iter().enumerate() {
            for (ci, control) in para.controls.iter().enumerate() {
                if let Control::Table(table) = control {
                    for (index, cell) in table.cells.iter().enumerate() {
                        if cell
                            .paragraphs
                            .iter()
                            .map(|p| p.text.as_str())
                            .collect::<String>()
                            == "월별누계"
                        {
                            source.push(((si, pi, ci), index, cell));
                        }
                    }
                }
            }
        }
    }
    assert_eq!(source.len(), 1, "원본의 월별누계 셀을 식별한다");
    let (owner, index, original) = source[0];
    let trees: Vec<_> = (0..core.page_count())
        .map(|p| core.build_page_render_tree(p).unwrap())
        .collect();
    let mut cells = Vec::new();
    for tree in &trees {
        locate(&tree.root, owner, false, index, &mut cells);
    }
    assert_eq!(cells.len(), 1, "원본 셀 내용의 누락·중복 방지");
    let cell = cells[0];
    let RenderNodeType::TableCell(meta) = &cell.node_type else {
        unreachable!()
    };
    assert_eq!((meta.row, meta.col), (original.row, original.col));
    assert_eq!(text(cell), "월별누계");
    let mut content_lines = Vec::new();
    lines(cell, &mut content_lines);
    assert_eq!(
        content_lines
            .iter()
            .map(|line| text(line))
            .collect::<Vec<_>>(),
        ["월별", "누계"],
        "독립 한컴 Print의 두 줄 순서"
    );
    for line in &content_lines {
        assert!(line.bbox.y >= cell.bbox.y - 0.5);
        assert!(
            line.bbox.y + line.bbox.height <= cell.bbox.y + cell.bbox.height + 0.5,
            "글줄은 소유 셀 안에 있어야 한다"
        );
        for run in &line.children {
            if matches!(run.node_type, RenderNodeType::TextRun(_)) && !text(run).is_empty() {
                assert!(run.bbox.x >= cell.bbox.x - 0.5);
                assert!(run.bbox.x + run.bbox.width <= cell.bbox.x + cell.bbox.width + 0.5);
            }
        }
    }
    assert!(
        content_lines[0].bbox.y + content_lines[0].bbox.height <= content_lines[1].bbox.y + 0.5
    );
}
