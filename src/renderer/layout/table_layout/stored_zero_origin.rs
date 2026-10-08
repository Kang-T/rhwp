//! A source paragraph can restart at zero inside the table's opening row frame.

use crate::model::paragraph::LineSeg;
use crate::model::shape::TextWrap;
use crate::model::table::{Cell, Table, TablePageBreak};

use super::{CellUnit, LayoutEngine, ResolvedStyleSet};

impl LayoutEngine {
    pub(crate) fn row_cut_ends_at_shared_zero_origin_frame(
        &self,
        table: &Table,
        row: usize,
        end_cut: &[usize],
        styles: &ResolvedStyleSet,
    ) -> bool {
        let mut cells: Vec<_> = table
            .cells
            .iter()
            .filter(|cell| cell.row as usize == row && cell.row_span == 1)
            .collect();
        cells.sort_by_key(|cell| cell.col);
        if cells.len() != end_cut.len() {
            return false;
        }
        let mut boundary_owner = false;
        for (cell, &end) in cells.iter().zip(end_cut) {
            let units = self.cell_units(cell, table, styles);
            if end >= units.len() {
                // A completed sibling has no next-frame owner to restart.
                continue;
            }
            if end != 1
                || !self.cut_starts_saved_zero_origin_text_frame(table, cell, &units, end)
                || !self.row_shared_zero_origin_frame_peer(table, cell)
            {
                return false;
            }
            boundary_owner = true;
        }
        boundary_owner
    }

    pub(super) fn row_shared_zero_origin_frame_peer(&self, table: &Table, cell: &Cell) -> bool {
        table.cells.iter().any(|peer| {
            if peer.row != cell.row
                || peer.col == cell.col
                || peer.row_span <= 1
                || peer.dirty_flag
                || peer.paragraphs.iter().any(|para| {
                    para.stored_text_partition_is_dirty()
                        || para.cell_format_vpos_dirty
                        || !para.controls.is_empty()
                })
            {
                return false;
            }
            let mut lines = peer.paragraphs.iter().flat_map(|para| &para.line_segs);
            let (Some(first), Some(second), Some(after)) =
                (lines.next(), lines.next(), lines.next())
            else {
                return false;
            };
            [first, second, after].iter().all(|line| {
                line.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                    && line.line_height > 0
                    && line.line_spacing >= 0
            }) && first.vertical_pos == 0
                && second.vertical_pos == 0
                && i64::from(after.vertical_pos)
                    == i64::from(second.line_height) + i64::from(second.line_spacing)
        })
    }

    pub(crate) fn row_has_saved_zero_origin_text_frame(&self, table: &Table, row: usize) -> bool {
        table
            .cells
            .iter()
            .filter(|cell| cell.row as usize == row && cell.row_span == 1)
            .any(|cell| {
                cell.paragraphs.iter().any(|para| {
                    para.line_segs.windows(2).any(|lines| {
                        self.stored_zero_origin_text_frame_break(table, cell, &lines[0], &lines[1])
                    })
                })
            })
    }

    pub(super) fn stored_zero_origin_text_frame_break(
        &self,
        table: &Table,
        cell: &Cell,
        previous: &LineSeg,
        next: &LineSeg,
    ) -> bool {
        let profile = self.profile.get();
        if !(profile.hwp5_stored_pagination_layout() || profile.hwpx_stored_layout())
            || profile.session_edited()
            || self
                .render_normalization
                .borrow()
                .table_text_reflowed(table)
            || table.common.treat_as_char
            || table.common.text_wrap != TextWrap::TopAndBottom
            || table.page_break != TablePageBreak::RowBreak
            || cell.row_span != 1
            || cell.dirty_flag
            || table.common.height == 0
            || table.common.height > i32::MAX as u32
            || (previous.tag | next.tag) & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
            || previous.vertical_pos != 0
            || next.vertical_pos != 0
            || previous.line_height <= 0
            || next.line_height <= 0
            || previous.line_spacing < 0
            || previous.text_start >= next.text_start
        {
            return false;
        }
        // A paragraph change can reset a writer-local cursor. Require adjacent
        // source lines in this paragraph, then a physical frame or an independent
        // spanning owner whose next source ladder confirms the same row boundary.
        if !cell.paragraphs.iter().any(|para| {
            !para.stored_text_partition_is_dirty()
                && !para.cell_format_vpos_dirty
                && para.controls.is_empty()
                && !para.text.trim().is_empty()
                && para
                    .line_segs
                    .windows(2)
                    .any(|lines| std::ptr::eq(&lines[0], previous) && std::ptr::eq(&lines[1], next))
        }) {
            return false;
        }
        if self.row_shared_zero_origin_frame_peer(table, cell) {
            return true;
        }
        // The original opening-frame witness owns positive trailing spacing.
        // A zero-spacing pair alone does not identify a physical page boundary.
        if previous.line_spacing == 0 {
            return false;
        }
        let rows = table.get_raw_row_heights();
        let row = usize::from(cell.row);
        let Some(prefix) = rows.get(..row) else {
            return false;
        };
        let Some(&height) = rows.get(row) else {
            return false;
        };
        if prefix
            .iter()
            .any(|&height| height == 0 || height > i32::MAX as u32)
            || height == 0
            || height > i32::MAX as u32
        {
            return false;
        }
        let start = prefix.iter().map(|&height| i64::from(height)).sum::<i64>()
            + i64::from(table.cell_spacing) * row as i64;
        let frame = i64::from(table.common.height);
        start < frame && frame < start + i64::from(height)
    }

    pub(super) fn cut_starts_saved_zero_origin_text_frame(
        &self,
        table: &Table,
        cell: &Cell,
        units: &[CellUnit],
        next: usize,
    ) -> bool {
        let Some(previous) = next.checked_sub(1).and_then(|index| units.get(index)) else {
            return false;
        };
        let Some(following) = units.get(next) else {
            return false;
        };
        if previous.para_idx != following.para_idx || previous.vis_end != following.vis_start {
            return false;
        }
        cell.paragraphs.get(previous.para_idx).is_some_and(|para| {
            para.line_segs
                .get(previous.vis_end.saturating_sub(1))
                .zip(para.line_segs.get(following.vis_start))
                .is_some_and(|(before, after)| {
                    self.stored_zero_origin_text_frame_break(table, cell, before, after)
                })
        })
    }
}
