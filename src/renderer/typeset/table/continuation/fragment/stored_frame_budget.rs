//! A genuine first fragment owns its declared physical space, not the next row.

use crate::renderer::typeset::{TypesetEngine, TypesetState};

use super::FragmentInput;

pub(super) struct StoredPartialFragment {
    pub(super) height: f64,
    pub(super) complete_last_row: Option<(usize, f64)>,
}

impl TypesetEngine {
    pub(super) fn stored_partial_fragment_height(
        &self,
        st: &TypesetState,
        input: FragmentInput<'_>,
        saved_frame: Option<(f64, f64)>,
        declared_height_is_credible: bool,
        available: f64,
    ) -> Option<StoredPartialFragment> {
        let source = input.source;
        if !declared_height_is_credible
            || !(st.profile.hwp5_stored_pagination_layout() || st.profile.hwpx_stored_layout())
            || st.profile.session_edited()
            || source.paragraph.stored_text_partition_is_dirty()
            || source.paragraph.cell_format_vpos_dirty
            || self.render_normalization.table_text_reflowed(source.table)
            || !input
                .prepared
                .layout_engine
                .stored_row_boxes_exceed_first_frame(source.row_geometry_table)
        {
            return None;
        }
        // The existing saved-frame producer authenticates the host origin and
        // available body space. The raw row lower bound independently proves
        // this object box is only a fragment. Its units still need a real cut.
        let height = saved_frame?.0.min(available);
        let mut before = 0.0;
        let mut complete_last_row = None;
        for (row, row_height) in input.prepared.cut_row_heights.iter().enumerate() {
            if row > 0 {
                before += input.prepared.cell_spacing;
            }
            if before + row_height > height {
                let remaining = height - before;
                // A declared row can include unused space. All of its units
                // may fit in the source remainder even when that box does not.
                // The complete-row minimum covers owners starting in this row.
                // A spanning owner can still owe units when its span ends here;
                // absence of a span after the cut does not prove completion.
                let active: Vec<_> = source
                    .row_geometry_table
                    .cells
                    .iter()
                    .filter(|cell| {
                        usize::from(cell.row) <= row
                            && row < usize::from(cell.row) + usize::from(cell.row_span)
                    })
                    .collect();
                let ordinary_owners = !active.is_empty()
                    && active
                        .iter()
                        .all(|cell| usize::from(cell.row) == row && cell.row_span == 1);
                let content = input
                    .prepared
                    .layout_engine
                    .row_complete_cut_content_height(source.row_geometry_table, row, source.styles);
                if ordinary_owners && content <= remaining {
                    complete_last_row = Some((row, remaining));
                }
                break;
            }
            before += row_height;
        }
        Some(StoredPartialFragment {
            height,
            complete_last_row,
        })
    }
}
