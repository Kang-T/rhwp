//! Source row boxes authenticate partial text frames beyond the opening fragment.

use crate::model::paragraph::LineSeg;
use crate::model::table::{Table, TablePageBreak};

use super::{LayoutEngine, ResolvedStyleSet};

impl LayoutEngine {
    pub(crate) fn stored_row_boxes_exceed_first_frame(&self, table: &Table) -> bool {
        if table.common.height == 0 || table.common.height > i32::MAX as u32 {
            return false;
        }
        // Rows without a single-row declaration contribute a zero lower bound.
        // A merged-only row cannot erase the independently declared other rows.
        let lower_bound: i64 = table
            .get_raw_row_heights()
            .iter()
            .filter(|&&height| height < 0x8000_0000)
            .map(|&height| i64::from(height))
            .sum();
        lower_bound + i64::from(table.cell_spacing) * i64::from(table.row_count.saturating_sub(1))
            > i64::from(table.common.height)
    }

    pub(crate) fn stored_two_line_row_cut_frame(
        &self,
        table: &Table,
        row: usize,
        styles: &ResolvedStyleSet,
    ) -> bool {
        let profile = self.profile.get();
        if !(profile.hwp5_stored_pagination_layout() || profile.hwpx_stored_layout())
            || profile.session_edited()
            || self
                .render_normalization
                .borrow()
                .table_text_reflowed(table)
            || table.common.treat_as_char
            || table.page_break != TablePageBreak::RowBreak
            || !self.stored_row_boxes_exceed_first_frame(table)
        {
            return false;
        }
        let cells: Vec<_> = table
            .cells
            .iter()
            .filter(|cell| usize::from(cell.row) == row)
            .collect();
        !cells.is_empty()
            && cells.iter().all(|cell| {
                cell.row_span == 1
                    && !cell.dirty_flag
                    && cell.paragraphs.iter().all(|para| {
                        !para.stored_text_partition_is_dirty()
                            && !para.cell_format_vpos_dirty
                            && para.controls.is_empty()
                    })
            })
            && cells.iter().any(|cell| {
                let [first, next] = cell.paragraphs.as_slice() else {
                    return false;
                };
                let ([before], [after]) = (first.line_segs.as_slice(), next.line_segs.as_slice())
                else {
                    return false;
                };
                let padding = cell.effective_padding(&table.padding);
                (before.tag | after.tag) & LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                    && before.vertical_pos == 0
                    && after.vertical_pos == 0
                    && before.line_height > 0
                    && after.line_height > 0
                    && before.line_spacing >= 0
                    && i64::from(before.line_height)
                        + i64::from(before.line_spacing)
                        + i64::from(after.line_height)
                        + i64::from(padding.top)
                        + i64::from(padding.bottom)
                        == i64::from(cell.height)
                    && [first, next].iter().all(|para| {
                        styles
                            .para_styles
                            .get(para.para_shape_id as usize)
                            .is_some_and(|style| {
                                style.spacing_before == 0.0 && style.spacing_after == 0.0
                            })
                    })
            })
    }
}
