//! The opening table frame and a later source cell use different vertical origins.

use crate::model::table::{Cell, Table};
use crate::renderer::hwpunit_to_px;

use super::{LayoutEngine, ResolvedStyleSet};

impl LayoutEngine {
    /// Normalize the original opening box into the ordinary source row. This
    /// only supplies the existing saved-cut tail calculation; accepted physical
    /// rows and the table's occupied extent remain in their original ledger.
    pub(super) fn stored_first_cell_frame_height(&self, table: &Table, cell: &Cell) -> Option<f64> {
        (cell.row_span == 1)
            .then(|| self.source_first_cell_frame_height(table, cell))
            .flatten()
    }

    fn source_first_cell_frame_height(&self, table: &Table, cell: &Cell) -> Option<f64> {
        if cell.row == 0
            || !self.stored_plain_text_cell_frame(table, cell)
            || table.common.height == 0
            || table.common.height > i32::MAX as u32
        {
            return None;
        }
        let rows = table.get_raw_row_heights();
        let row = usize::from(cell.row);
        let prefix = rows.get(..row)?;
        let &current = rows.get(row)?;
        if prefix
            .iter()
            .chain(std::iter::once(&current))
            .any(|&height| height == 0 || height > i32::MAX as u32)
        {
            return None;
        }
        let origin = prefix.iter().map(|&height| i64::from(height)).sum::<i64>()
            + i64::from(table.cell_spacing) * row as i64;
        let end = i64::from(table.common.height);
        // A completed opening row or a later frame cannot authenticate this
        // partial source row. Unknown zero-height rows are not a physical origin.
        if end <= origin || end >= origin + i64::from(current) {
            return None;
        }
        Some(hwpunit_to_px(i32::try_from(end - origin).ok()?, self.dpi))
    }

    fn first_source_frame_owner(
        &self,
        table: &Table,
        cell: &Cell,
        styles: &ResolvedStyleSet,
    ) -> Option<(usize, f64)> {
        let opening = self.source_first_cell_frame_height(table, cell)?;
        let units = self.cell_units(cell, table, styles);
        let padding = cell.effective_padding(&table.padding);
        let padding = hwpunit_to_px(i32::from(padding.top) + i32::from(padding.bottom), self.dpi);
        let epsilon = hwpunit_to_px(2, self.dpi);
        let mut previous: Option<&crate::model::paragraph::LineSeg> = None;
        let mut advance = 0.0;
        for (index, unit) in units.iter().enumerate() {
            if unit.vis_end != unit.vis_start + 1 {
                return None;
            }
            let line = cell
                .paragraphs
                .get(unit.para_idx)?
                .line_segs
                .get(unit.vis_start)?;
            if let Some(before) = previous {
                if line.vertical_pos == 0 && before.vertical_pos > 0 {
                    let ink = hwpunit_to_px(
                        before.vertical_pos.checked_add(before.line_height)?,
                        self.dpi,
                    ) + padding;
                    // A stored empty line owns the same line box and following
                    // advance as text. The opening box ends in that final gap.
                    return (opening + epsilon >= ink && opening < advance + padding)
                        .then_some((index, opening));
                }
                if i64::from(line.vertical_pos)
                    != i64::from(before.vertical_pos)
                        + i64::from(before.line_height)
                        + i64::from(before.line_spacing)
                {
                    return None;
                }
            } else if line.vertical_pos != 0 {
                return None;
            }
            let source_advance =
                hwpunit_to_px(line.line_height.checked_add(line.line_spacing)?, self.dpi);
            if (unit.height - source_advance).abs() > epsilon {
                return None;
            }
            advance += unit.height;
            previous = Some(line);
        }
        None
    }

    pub(super) fn stored_span_partial_first_frame_owner(
        &self,
        table: &Table,
        cell: &Cell,
        row: usize,
        cut: &[usize],
        measured_rows: &[f64],
        styles: &ResolvedStyleSet,
    ) -> Option<usize> {
        if cell.row_span <= 1 || usize::from(cell.row) != row || cut.is_empty() {
            return None;
        }
        let (owner, opening) = self.first_source_frame_owner(table, cell, styles)?;
        let epsilon = hwpunit_to_px(2, self.dpi);
        let origin = measured_rows.get(..row)?.iter().sum::<f64>()
            + hwpunit_to_px(table.cell_spacing as i32, self.dpi) * row as f64;
        if (origin + opening - hwpunit_to_px(table.common.height as i32, self.dpi)).abs() > epsilon
            || (self.row_cut_content_height(table, row, &[], cut, styles) - opening).abs() > epsilon
        {
            return None;
        }
        table
            .cells
            .iter()
            .any(|peer| {
                peer.row == cell.row
                    && peer.col != cell.col
                    && self
                        .first_source_frame_owner(table, peer, styles)
                        .is_some_and(|(_, height)| (height - opening).abs() <= epsilon)
            })
            .then_some(owner)
    }

    /// A source cut consumes the opening physical box, not the row's entire
    /// trailing band. The next fragment reserves and paints this same remainder.
    pub(crate) fn stored_first_frame_row_remainder(
        &self,
        table: &Table,
        row: usize,
        cut: &[usize],
        measured_rows: &[f64],
        styles: &ResolvedStyleSet,
    ) -> Option<f64> {
        let mut cells: Vec<_> = table
            .cells
            .iter()
            .filter(|cell| usize::from(cell.row) == row && cell.row_span == 1)
            .collect();
        cells.sort_by_key(|cell| cell.col);
        if cells.len() != cut.len() || row >= measured_rows.len() {
            return None;
        }
        let epsilon = hwpunit_to_px(2, self.dpi);
        let mut opening: Option<f64> = None;
        for (cell, &end) in cells.into_iter().zip(cut) {
            let units = self.cell_units(cell, table, styles);
            if end >= units.len() {
                continue;
            }
            if end == 0 {
                return None;
            }
            let height = self.stored_first_cell_frame_height(table, cell)?;
            let before = units[..end - 1].iter().map(|unit| unit.height).sum();
            if self.native_intra_para_saved_reset_trailing_trim(
                table,
                cell,
                &units,
                end,
                before,
                units[end - 1].height,
            ) <= 0.0
                || opening.is_some_and(|other| (other - height).abs() > epsilon)
            {
                return None;
            }
            opening = Some(height);
        }
        let opening = opening?;
        let origin = measured_rows[..row].iter().sum::<f64>()
            + hwpunit_to_px(table.cell_spacing as i32, self.dpi) * row as f64;
        let source_end = hwpunit_to_px(table.common.height as i32, self.dpi);
        let accepted = self.row_cut_content_height(table, row, &[], cut, styles);
        // Source and measured coordinates must identify the same opening box.
        // A grown prefix cannot be replaced with raw row minima to certify it.
        if (origin + opening - source_end).abs() > epsilon || (accepted - opening).abs() > epsilon {
            return None;
        }
        let remaining = measured_rows[row] - opening;
        let content = self.row_cut_content_height(table, row, cut, &[], styles);
        (remaining > content + epsilon).then_some(remaining)
    }
}
