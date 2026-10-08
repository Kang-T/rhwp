//! Stored rowspan minimums consume physical boxes accepted by earlier fragments.

use crate::renderer::typeset::{
    hwpunit_to_px, BlockTableContinuationPreparedState, BlockTableContinuationSource,
    TypesetEngine, TypesetState,
};

use super::super::TableContinuationCursor;
use super::FragmentInput;

impl TypesetEngine {
    pub(super) fn stored_header_fragment_row_budget(
        &self,
        st: &TypesetState,
        input: FragmentInput<'_>,
        available: f64,
        outer_top: f64,
    ) -> f64 {
        if !input.start.is_continuation || !self.stored_header_physical_rows(st, input.source) {
            return available;
        }
        // Paint closes a page-filling fragment at this same physical boundary.
        // Its accepted row boxes must exclude the unpainted bottom band before
        // a later closing span subtracts those boxes from its declared height.
        let bottom = crate::renderer::float_placement::single_cell_page_fragment_bottom(
            input.source.table,
            st.layout.body_area.height,
            self.dpi,
        );
        available.min(
            (bottom - st.current_zone_y_offset - st.layout.pagination_tolerance_px - outer_top)
                .max(0.0),
        )
    }

    pub(super) fn stored_header_physical_rows(
        &self,
        st: &TypesetState,
        source: BlockTableContinuationSource<'_>,
    ) -> bool {
        crate::renderer::float_placement::stored_repeated_header_rowbreak_outer_frame(
            st.profile,
            source.paragraph,
            source.table,
        ) && !self.render_normalization.table_text_reflowed(source.table)
    }

    pub(super) fn stored_fragment_row_plan(
        &self,
        st: &TypesetState,
        prepared: &BlockTableContinuationPreparedState,
        cursor: &TableContinuationCursor,
        source: BlockTableContinuationSource<'_>,
    ) -> Option<Vec<f64>> {
        let keeps_source_boxes = self.stored_header_physical_rows(st, source)
            || prepared
                .layout_engine
                .has_stored_continuous_spans(source.row_geometry_table);
        if !keeps_source_boxes
            || !cursor.start_cut.is_empty()
            || cursor.start_row_height_override.is_some()
            || !cursor.accepted_row_boxes.iter().any(Option::is_some)
        {
            return None;
        }
        let table = source.row_geometry_table;
        let original = &source.measured_table.row_heights;
        let mut heights = original.clone();
        for (row, accepted) in cursor
            .accepted_row_boxes
            .iter()
            .enumerate()
            .take(cursor.row)
        {
            if let (Some(height), Some(target)) = (accepted, heights.get_mut(row)) {
                *target = *height;
            }
        }
        for row in cursor.row..heights.len() {
            // Only a closing span whose preceding physical boxes changed can
            // invalidate the globally measured remainder. Other rows keep their
            // original measurement, including content-driven growth.
            let closing: Vec<_> = table
                .cells
                .iter()
                .filter(|cell| {
                    cell.row_span > 1
                        && cell.height < 0x8000_0000
                        && cell.row as usize + cell.row_span as usize == row + 1
                })
                .collect();
            if !closing.iter().any(|cell| {
                let first = cell.row as usize;
                (first..cursor.row.min(row))
                    .any(|prior| (heights[prior] - original[prior]).abs() > 0.5)
            }) {
                continue;
            }
            // Whole-row content and single-row declarations provide the floor.
            // Rows with no single-row owner retain the original unknown-row
            // distribution rather than inventing a new one here.
            if !table
                .cells
                .iter()
                .any(|cell| cell.row as usize == row && cell.row_span == 1)
            {
                continue;
            }
            let floor =
                prepared
                    .layout_engine
                    .row_cut_content_height(table, row, &[], &[], source.styles);
            let required = closing.iter().fold(floor, |height, cell| {
                let first = cell.row as usize;
                let used = heights[first..row].iter().sum::<f64>()
                    + prepared.cell_spacing * (row - first) as f64;
                height.max(hwpunit_to_px(cell.height as i32, self.dpi) - used)
            });
            heights[row] = required;
        }
        Some(heights)
    }

    pub(super) fn record_stored_fragment_rows(
        &self,
        st: &TypesetState,
        cursor: &mut TableContinuationCursor,
        input: FragmentInput<'_>,
        end_row: usize,
        complete_end_cut: bool,
        last_height: Option<f64>,
    ) {
        let keeps_source_boxes = self.stored_header_physical_rows(st, input.source)
            || input
                .prepared
                .layout_engine
                .has_stored_continuous_spans(input.source.row_geometry_table);
        if !keeps_source_boxes
            || !input.start.start_cut.is_empty()
            || input.start.start_row_height_override.is_some()
            || !complete_end_cut
        {
            return;
        }
        let heights = input
            .fragment_row_heights
            .unwrap_or(&input.source.measured_table.row_heights);
        cursor.accepted_row_boxes.resize(heights.len(), None);
        for (row, &height) in heights
            .iter()
            .enumerate()
            .take(end_row)
            .skip(input.start.cursor_row)
        {
            cursor.accepted_row_boxes[row] = Some(if row + 1 == end_row {
                last_height.unwrap_or(height)
            } else {
                height
            });
        }
    }
}
