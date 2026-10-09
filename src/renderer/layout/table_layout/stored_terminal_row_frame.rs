//! Complete original rows retain their source padding and terminal ownership.

use crate::model::table::Table;
use crate::renderer::hwpunit_to_px;

use super::{LayoutEngine, ResolvedStyleSet, ROW_CUT_CAPACITY_FP_EPSILON_PX};

impl LayoutEngine {
    /// A whole source row loses only padding when its small cellSz is used
    /// for a content cut. Its actual grown frame belongs to both scan and paint.
    pub(crate) fn stored_whole_row_padding_frame(
        &self,
        table: &Table,
        row: usize,
        cut: f64,
        painted: f64,
    ) -> bool {
        self.whole_fragment_row_uses_measured_height(table, row)
            && self.whole_row_height_diff_is_padding_reduction(table, row, cut, painted)
            && table
                .cells
                .iter()
                .filter(|cell| cell.row as usize == row)
                .all(|cell| {
                    cell.row_span == 1
                        && self.stored_plain_text_cell_frame(table, cell)
                        && crate::renderer::cell_vpos_ladder_is_intact(&cell.paragraphs)
                })
    }

    /// The last ordinary source row can fit while a spanning cell's cached
    /// blank minimum raises only that row. Complete every closing owner in
    /// its accepted physical box before using the ordinary source boundary.
    /// Interior rows and spans already cut on an earlier page are excluded.
    pub(crate) fn stored_complete_terminal_row_frame(
        &self,
        table: &Table,
        row: usize,
        first_row: usize,
        row_heights: &[f64],
        available: f64,
        styles: &ResolvedStyleSet,
    ) -> Option<f64> {
        if row + 1 != usize::from(table.row_count)
            || row <= first_row
            || row >= row_heights.len()
            || !available.is_finite()
            || available <= 0.0
        {
            return None;
        }
        let ordinary: Vec<_> = table
            .cells
            .iter()
            .filter(|cell| cell.row as usize == row && cell.row_span == 1)
            .collect();
        if ordinary.is_empty()
            || ordinary
                .iter()
                .any(|cell| cell.height == 0 || cell.height > i32::MAX as u32)
        {
            return None;
        }
        let source = ordinary
            .iter()
            .map(|cell| hwpunit_to_px(cell.height as i32, self.dpi))
            .fold(0.0_f64, f64::max);
        if source > available + ROW_CUT_CAPACITY_FP_EPSILON_PX
            || self.row_complete_cut_content_height(table, row, styles) > available
        {
            return None;
        }
        let spacing = hwpunit_to_px(table.cell_spacing as i32, self.dpi);
        let mut has_closing_span = false;
        for cell in table.cells.iter().filter(|cell| {
            cell.row as usize <= row && row < cell.row as usize + cell.row_span as usize
        }) {
            let first = usize::from(cell.row);
            if first < first_row
                || !self.stored_plain_text_cell_frame(table, cell)
                || !crate::renderer::cell_vpos_ladder_is_intact(&cell.paragraphs)
            {
                return None;
            }
            let units = self.cell_units(cell, table, styles);
            if units.is_empty()
                || units
                    .iter()
                    .any(|unit| unit.hard_break_before || unit.page_frame_reset_before)
            {
                return None;
            }
            let physical = row_heights[first..row].iter().sum::<f64>()
                + spacing * (row - first) as f64
                + source.min(available);
            if self.cell_cut_visible_height(cell, table, styles, 0, units.len()) > physical {
                return None;
            }
            has_closing_span |= cell.row_span > 1;
        }
        has_closing_span.then_some(source.min(available))
    }
}
