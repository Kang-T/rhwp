//! 원본 빈 호스트의 통째 자리차지 표가 예약한 바깥 상자를 배치에도 전달한다.

use crate::model::{
    control::Control,
    paragraph::{LineSeg, Paragraph},
    shape::{CommonObjAttr, HorzRelTo, TextWrap, VertRelTo},
    table::Table,
};
use crate::renderer::{
    float_placement::{
        is_para_topbottom_float, signed_hwpunit, ParagraphFloatFlow, ParagraphFloatPlacement,
    },
    hwpunit_to_px,
};

pub(super) struct FrameInput {
    pub original: bool,
    pub anchor: f64,
    pub height: f64,
    pub before: f64,
    pub after: f64,
    pub dpi: f64,
}

/// 종이·쪽에 고정된 전면/배경 장식은 보이더라도 호스트의 글줄 흐름을 밀지 않는다.
/// 인라인 개체나 본문과 어울리는 개체는 같은 단독 표 상자로 합치지 않는다.
fn detached_overlay(common: &CommonObjAttr) -> bool {
    !common.treat_as_char
        && !common.flow_with_text
        && !common.affect_line_spacing
        && matches!(
            common.text_wrap,
            TextWrap::InFrontOfText | TextWrap::BehindText
        )
        && matches!(common.vert_rel_to, VertRelTo::Paper | VertRelTo::Page)
        && matches!(common.horz_rel_to, HorzRelTo::Paper | HorzRelTo::Page)
}

fn has_single_flow_table(para: &Paragraph) -> bool {
    para.controls
        .iter()
        .filter(|c| matches!(c, Control::Table(_)))
        .count()
        == 1
        && para.controls.iter().all(|control| match control {
            Control::Table(_) => true,
            Control::Shape(shape) => detached_overlay(shape.common()),
            Control::Picture(picture) => detached_overlay(&picture.common),
            _ => false,
        })
}

/// 분할 조각의 계약을 바꾸지 않는다. 기존 통째 적합성 후보만 같은 위·아래 여백
/// 상자를 소비한다. 자란 내용·수정된 줄·캡션·여러 흐름 개체는 별도 경로가 소유한다.
pub(super) fn prepare(
    para: &Paragraph,
    table: &Table,
    frame: FrameInput,
) -> Option<ParagraphFloatPlacement> {
    if !frame.original
        || !is_para_topbottom_float(&table.common)
        || table.caption.is_some()
        || para.stored_text_partition_is_dirty()
        || para.cell_format_vpos_dirty
        || !para.text.trim().is_empty()
        || !has_single_flow_table(para)
        || para.line_segs.len() != 1
        || para.line_segs[0].tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
        || table.common.height > i32::MAX as u32
        || signed_hwpunit(table.common.vertical_offset) < 0
        || (frame.height - hwpunit_to_px(table.common.height as i32, frame.dpi)).abs()
            > hwpunit_to_px(2, frame.dpi)
    {
        return None;
    }
    let table_top = frame.anchor
        + frame.before
        + hwpunit_to_px(signed_hwpunit(table.common.vertical_offset), frame.dpi);
    Some(ParagraphFloatPlacement {
        flow: ParagraphFloatFlow::EmptyHost,
        anchor_y: frame.anchor,
        stored_host_origin: None,
        stored_successor_line_origin: None,
        table_left: None,
        table_top,
        occupied_bottom: table_top + frame.height + frame.after,
    })
}
