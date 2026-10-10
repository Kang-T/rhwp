//! 첫 저장 글줄의 앞 간격을 측정과 실제 배치의 같은 쪽 원점으로 해석한다.

use crate::model::{control::Control, paragraph::Paragraph};
use crate::renderer::{
    hwpunit_to_px, layout::para_has_overlay_shape, style_resolver::ResolvedStyleSet,
};

/// 쪽 첫 저장 줄의 vpos가 문단 앞 간격을 포함하면 쪽 원점은 0이다.
/// 다음 문단의 저장 사다리도 앞 간격을 계상하는지 확인하여 측정과 배치가
/// 같은 원점을 소비한다. 명시적 쪽 나눔과 합성 줄은 이 증거로 사용하지 않는다.
pub(crate) fn stored_first_margin_is_page_relative(
    first: &Paragraph,
    next: Option<&Paragraph>,
    styles: &ResolvedStyleSet,
    dpi: f64,
) -> bool {
    if first.column_type == crate::model::paragraph::ColumnBreakType::Page {
        return false;
    }
    let Some(first_seg) = first.line_segs.first() else {
        return false;
    };
    let Some(last_seg) = first.line_segs.last() else {
        return false;
    };
    let Some(next) = next else {
        return false;
    };
    let Some(next_seg) = next.line_segs.first() else {
        return false;
    };
    let first_before = styles
        .para_styles
        .get(first.para_shape_id as usize)
        .map(|style| style.spacing_before)
        .unwrap_or(0.0);
    let next_before = styles
        .para_styles
        .get(next.para_shape_id as usize)
        .map(|style| style.spacing_before)
        .unwrap_or(0.0);
    // 다음 앞 간격이 0이어도 저장 전진 끝과 정확히 이어지면 같은 쪽 상대 사다리다.
    // 빈 첫 문단의 유효한 앞 간격도 줄 점유에 포함되며 쪽 원점으로 빼지 않는다.
    // 글자처럼 취급되는 도형이 있는 첫 문단은 제목과 개체가 저장 원점을 함께
    // 소유한다. 문단 앞 여백만으로 쪽 원점을 다시 정하면 후속 표가 그 여백만큼
    // 위로 이동한다. 일반 텍스트 문단의 여백 사다리와 구분한다(#6797 7쪽).
    !first
        .controls
        .iter()
        .any(|control| matches!(control, Control::Shape(shape) if shape.common().treat_as_char))
        && !para_has_overlay_shape(first)
        && !para_has_overlay_shape(next)
        && first_seg.tag & crate::model::paragraph::LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
        && last_seg.tag & crate::model::paragraph::LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
        && next_seg.tag & crate::model::paragraph::LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
        && first_before > 0.5
        && next_before >= 0.0
        && (hwpunit_to_px(first_seg.vertical_pos, dpi) - first_before).abs() <= 0.5
        && (hwpunit_to_px(
            next_seg.vertical_pos
                - last_seg.vertical_pos
                - last_seg.line_height
                - last_seg.line_spacing,
            dpi,
        ) - next_before)
            .abs()
            <= 0.5
}
