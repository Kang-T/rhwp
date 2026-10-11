//! 저장 TAC 줄이 소유한 중첩 표를 행으로 펼쳐도 원래 쪽 시작을 보존한다.

use crate::model::control::Control;
use crate::model::paragraph::LineSeg;
use crate::model::table::{Cell, Table, TablePageBreak};

use super::{CellUnit, LayoutEngine, ResolvedStyleSet};

impl LayoutEngine {
    /// 원본 한 줄의 상자가 자식 표 전체와 일치할 때만 그 행 유닛의 합을 돌려준다.
    /// 같은 원장은 측정과 실제 부분 배치가 소비하므로 선언 높이로 컷을 다시 만들지 않는다.
    pub(super) fn stored_inline_row_frame_height(
        &self,
        table: &Table,
        cell: &Cell,
        units: &[CellUnit],
        index: usize,
        styles: &ResolvedStyleSet,
    ) -> Option<f64> {
        if !self.profile.get().hwpx_stored_layout()
            || self.profile.get().session_edited()
            || self
                .render_normalization
                .borrow()
                .table_text_reflowed(table)
            || table.common.treat_as_char
            || !matches!(table.page_break, TablePageBreak::RowBreak)
            || cell.row_span != 1
            || cell.dirty_flag
            || cell.col != 0
            || cell.col_span != table.col_count
            || Self::row_has_prior_rowspan_cover(table, usize::from(cell.row))
        {
            return None;
        }
        let first = units.get(index)?;
        if !first.page_frame_reset_before || first.nested_row != Some(0) {
            return None;
        }
        let paragraph = cell.paragraphs.get(first.para_idx)?;
        if paragraph.stored_text_partition_is_dirty()
            || paragraph.cell_format_vpos_dirty
            || !paragraph.text.trim().is_empty()
            || paragraph.line_segs.len() != 1
            || paragraph.controls.len() != 1
        {
            return None;
        }
        let Control::Table(child) = &paragraph.controls[0] else {
            return None;
        };
        let owner = super::super::control_line_seg_index(paragraph, 0)?;
        let line = paragraph.line_segs.get(owner)?;
        if !child.common.treat_as_char
            || child.common.height == 0
            || line.line_height <= 0
            || line.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
        {
            return None;
        }
        // 한컴이 저장한 객체 높이와 바깥여백이 줄 상자를 닫아야 한다.
        // 합성 줄이나 내용 때문에 더 커진 표를 한 쪽짜리로 취급하지 않는다.
        let object_height = i64::from(child.common.height)
            + i64::from(child.outer_margin_top)
            + i64::from(child.outer_margin_bottom);
        if (i64::from(line.line_height) - object_height).abs() > 2 {
            return None;
        }
        let group: Vec<_> = units[index..]
            .iter()
            .take_while(|unit| unit.para_idx == first.para_idx && unit.nested_row.is_some())
            .collect();
        if group.len() != usize::from(child.row_count)
            || !group
                .iter()
                .enumerate()
                .all(|(row, unit)| unit.nested_row == Some(row))
        {
            return None;
        }
        let height: f64 = group.iter().map(|unit| unit.height).sum();
        let spacing = styles
            .para_styles
            .get(paragraph.para_shape_id as usize)
            .map_or(0.0, |style| style.spacing_before + style.spacing_after);
        let stored_height = crate::renderer::hwpunit_to_px(line.line_height, self.dpi);
        (height.is_finite()
            && height > 0.0
            && height <= stored_height + spacing + super::ROW_CUT_CAPACITY_FP_EPSILON_PX)
            .then_some(height)
    }
}
