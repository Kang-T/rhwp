//! A repeated title row starts a new outer frame at each stored table fragment.

use crate::model::paragraph::{LineSeg, Paragraph};
use crate::model::provenance::LayoutCompatibilityProfile;
use crate::model::shape::{HorzRelTo, VertRelTo};
use crate::model::table::{Table, TablePageBreak};

pub(crate) fn stored_repeated_header_rowbreak_outer_frame(
    profile: LayoutCompatibilityProfile,
    para: &Paragraph,
    table: &Table,
) -> bool {
    (profile.hwp5_stored_pagination_layout() || profile.hwpx_stored_layout())
        && !profile.session_edited()
        && !para.stored_text_partition_is_dirty()
        && !para.cell_format_vpos_dirty
        && !para.line_segs.is_empty()
        && para.line_segs.iter().all(|line| {
            line.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                && line.vertical_pos >= 0
                && line.line_height > 0
        })
        && !table.common.treat_as_char
        && super::is_para_topbottom_float(&table.common)
        && table.common.vert_rel_to == VertRelTo::Para
        && table.common.horz_rel_to == HorzRelTo::Column
        && table.page_break == TablePageBreak::RowBreak
        && table.repeat_header
        && !table.leading_header_rows().is_empty()
}
