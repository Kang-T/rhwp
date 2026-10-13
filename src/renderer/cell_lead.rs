//! Stored leading space belongs to a cell's content occupancy once. Paragraph
//! positioning can consume the stored origin directly; row/cut height still
//! has to reserve that same space. Reflow has no saved origin to reuse.

use crate::model::paragraph::Paragraph;

pub(crate) fn cell_paragraph_leading_height(
    para: &Paragraph,
    para_index: usize,
    spacing_before: f64,
    dpi: f64,
) -> f64 {
    if para_index == 0 && !super::para_has_no_stored_line_segs(para) {
        super::cell_first_para_stored_lead(para, spacing_before, dpi)
    } else {
        super::cell_paragraph_spacing_before(para, para_index, spacing_before)
    }
}
