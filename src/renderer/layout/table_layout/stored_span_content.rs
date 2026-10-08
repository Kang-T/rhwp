//! Stored text spans distinguish a line's occupied box from its next-line advance.

use crate::model::paragraph::LineSeg;
use crate::model::table::{Cell, Table, TablePageBreak};
use crate::renderer::hwpunit_to_px;

use super::{CellUnit, LayoutEngine, ResolvedStyleSet};

impl LayoutEngine {
    pub(in crate::renderer::layout) fn rowbreak_prior_row_height(
        &self,
        table: &Table,
        row: usize,
        resolved_row_heights: &[f64],
        styles: &ResolvedStyleSet,
        stored_physical_span: bool,
    ) -> f64 {
        let physical = resolved_row_heights.get(row).copied().unwrap_or(0.0);
        // The previous fragment accepted this physical box. Re-measuring its
        // single-row siblings discards the trailing band and repeats span units.
        if stored_physical_span {
            return physical;
        }
        let has_single_row_cells = table
            .cells
            .iter()
            .any(|cell| cell.row as usize == row && cell.row_span == 1);
        let content = if has_single_row_cells {
            self.row_cut_content_height(table, row, &[], &[], styles)
        } else {
            0.0
        };
        if content > 0.0 {
            content
        } else {
            physical
        }
    }

    pub(in crate::renderer::layout) fn stored_plain_text_span_frame(
        &self,
        table: &Table,
        cell: &Cell,
    ) -> bool {
        let profile = self.profile.get();
        (profile.hwp5_stored_pagination_layout() || profile.hwpx_stored_layout())
            && !profile.session_edited()
            && !self
                .render_normalization
                .borrow()
                .table_text_reflowed(table)
            && !table.common.treat_as_char
            && table.page_break == TablePageBreak::RowBreak
            && cell.row_span > 1
            && !cell.dirty_flag
            && !cell.paragraphs.is_empty()
            && cell.paragraphs.iter().all(|para| {
                !para.stored_text_partition_is_dirty()
                    && !para.cell_format_vpos_dirty
                    && para.controls.is_empty()
                    && !para.line_segs.is_empty()
                    && para.line_segs.iter().all(|line| {
                        line.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                            && line.vertical_pos >= 0
                            && line.line_height > 0
                            && line.line_spacing >= 0
                    })
            })
    }

    pub(super) fn stored_span_line_tail_spacing(&self, cell: &Cell, unit: &CellUnit) -> f64 {
        if unit.empty_spacer
            || unit.vis_start >= unit.vis_end
            || unit.nested_row.is_some()
            || unit.mixed_nested_fragment
            || unit.non_inline_control_range.is_some()
        {
            return 0.0;
        }
        cell.paragraphs
            .get(unit.para_idx)
            .and_then(|para| para.line_segs.get(unit.vis_end - 1))
            .map(|line| hwpunit_to_px(line.line_spacing, self.dpi).min(unit.height.max(0.0)))
            .unwrap_or(0.0)
    }
}
