//! 관계형 — "한 줄로 입력" 칸의 편집 재조판은 줄을 나누지 않고, 같은 글의 보통 칸은 나눈다.
//! 주석 갱신: 2026-11-04T02:40:00+09:00 (새 파일)
//! 시각 측정은 해당 없음: 합성 문서의 편집 재조판 계약(줄 수·발행 폭)만 검사한다.
//! 근거는 한컴 저장 조판이다 — 공개 표본 1,205개를 편집 재조판으로 다시 재면 `lineWrap=SQUEEZE`
//! 칸 문단 5,192개가 한컴은 한 줄, rhwp 는 두 줄 이상이었다(예: `samples/hwpx/hancom-hwp/2024년 연간
//! 해외직접투자 보도자료 _ ff.hwp` 의 `1분기` 칸 — 저장 `segment_width` 2908, 한컴 PDF 글자 폭 29.04pt).
//! 칸 줄바꿈 방식을 반영하면 그 5,192개가 모두 맞고 새로 어긋나는 문단은 0개였다.

#![cfg(not(target_arch = "wasm32"))]

use rhwp::document_core::DocumentCore;
use rhwp::model::control::Control;
use rhwp::model::table::CELL_LINE_WRAP_SQUEEZE;

/// 칸 하나의 표에 상자보다 긴 글을 넣는다. `line_wrap` 은 칸 줄바꿈 방식이다.
fn edited_cell_rows(line_wrap: u8) -> Vec<(u32, i32)> {
    let mut core = DocumentCore::new_empty();
    core.create_blank_document_native()
        .expect("유효한 글자·문단 모양 정의");
    core.create_table_native(0, 0, 0, 1, 1).expect("1×1 표");
    let table_para = core.document().sections[0]
        .paragraphs
        .iter()
        .position(|p| p.controls.iter().any(|c| matches!(c, Control::Table(_))))
        .expect("표 문단");
    let mut document = core.document().clone();
    let Some(Control::Table(table)) = document.sections[0].paragraphs[table_para]
        .controls
        .iter_mut()
        .find(|c| matches!(c, Control::Table(_)))
    else {
        unreachable!()
    };
    table.cells[0].line_wrap = line_wrap;
    core.set_document(document);
    let ctrl = core.document().sections[0].paragraphs[table_para]
        .controls
        .iter()
        .position(|c| matches!(c, Control::Table(_)))
        .unwrap();
    // 글자 넣기가 편집 재조판을 부른다.
    core.insert_text_in_cell_native(0, table_para, ctrl, 0, 0, 0, &"가나다라마바사아자차".repeat(12))
        .expect("칸 글 삽입");
    let Some(Control::Table(table)) = core.document().sections[0].paragraphs[table_para]
        .controls
        .get(ctrl)
    else {
        unreachable!()
    };
    table.cells[0].paragraphs[0]
        .line_segs
        .iter()
        .map(|line| (line.text_start, line.segment_width))
        .collect()
}

#[test]
fn squeeze_cell_keeps_one_row_with_the_cell_box_width() {
    let wrapped = edited_cell_rows(0);
    let squeezed = edited_cell_rows(CELL_LINE_WRAP_SQUEEZE);
    assert!(wrapped.len() >= 2, "전제: 보통 칸은 상자 폭에서 줄을 나눈다: {wrapped:?}");
    assert_eq!(squeezed.len(), 1, "한 줄로 입력 칸은 한 줄이다: {squeezed:?}");
    assert_eq!(squeezed[0].0, 0);
    assert_eq!(
        squeezed[0].1, wrapped[0].1,
        "발행 폭은 넓힌 폭이 아니라 같은 칸의 안쪽 폭이다"
    );
}
