//! 저장 LINE_SEG 와 편집 재조판 결과의 일치도 측정.
//!
//! 한컴이 저장한 문서의 `PARA_LINE_SEG` 는 한컴이 확정한 줄 나눔·줄 높이다. rhwp 는 저장값이
//! 유효하면 그대로 그리므로, 열기만 해서는 rhwp 자신의 조판이 한컴과 같은지 드러나지 않는다.
//! 편집이 일어나면 그 문단은 [`reflow_line_segs`] 로 다시 계산되는데, 이 값이 저장값과
//! 다르면 편집한 문단부터 한컴과 다른 줄·높이·쪽수가 된다.
//!
//! 이 모듈은 모든 문단(본문, 표 셀, 중첩 표 셀)에 대해 편집 경로와 같은 상자로 재계산하고
//! 저장값과 필드별로 비교한다. 재계산은 복제본에서 하므로 문서는 바뀌지 않는다.
//! [`DocumentCore::forced_reflow_page_count`] 만 실제 문서에 재계산 결과를 적용한다.

use crate::document_core::DocumentCore;
use crate::model::control::Control;
use crate::model::paragraph::{LineSeg, Paragraph};
use crate::renderer::composer::{reflow_line_segs, ParagraphBox};
use crate::renderer::hwpunit_to_px;
use crate::renderer::page_layout::PageLayoutInfo;
use crate::renderer::style_resolver::ResolvedStyleSet;

/// 문단 하나의 비교 결과.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinesegAgreementRow {
    /// `body` 또는 `cell`
    pub scope: &'static str,
    pub section: usize,
    /// 본문 문단 번호 (셀이면 표를 가진 최외곽 본문 문단)
    pub para: usize,
    /// 셀 경로 — `컨트롤.셀.문단` 을 `>` 로 잇는다. 본문이면 빈 문자열.
    pub path: String,
    /// 표 중첩 깊이 (본문 0, 셀 1, 셀 안 표의 셀 2 …)
    pub depth: usize,
    pub chars: usize,
    /// 문단 모양의 세로 정렬 (0 글꼴 기준, 1 위, 2 가운데, 3 아래)
    pub vertical_alignment: u8,
    pub stored_lines: usize,
    pub reflowed_lines: usize,
    /// 맞대어 본 줄 중 줄 시작 글자(text_start)가 다른 줄 수
    pub break_mismatch: usize,
    pub line_height_mismatch: usize,
    pub text_height_mismatch: usize,
    pub baseline_mismatch: usize,
    pub line_spacing_mismatch: usize,
    /// 첫 줄 기준 상대 vpos 가 다른 줄 수
    pub vpos_mismatch: usize,
    pub max_line_height_delta: i32,
    pub max_baseline_delta: i32,
    pub max_vpos_delta: i32,
    /// 첫 줄 위부터 마지막 줄 아래까지 (HWPUNIT, 마지막 줄의 줄간격 포함)
    pub stored_height: i32,
    pub reflowed_height: i32,
    /// 처음 어긋난 줄 — `줄번호: 저장 ts/lh/th/bl/ls => 재계산 ts/lh/th/bl/ls`. 모두 같으면 빈 문자열.
    pub first_diff: String,
}

impl LinesegAgreementRow {
    pub fn all_match(&self) -> bool {
        self.stored_lines == self.reflowed_lines
            && self.break_mismatch == 0
            && self.line_height_mismatch == 0
            && self.text_height_mismatch == 0
            && self.baseline_mismatch == 0
            && self.line_spacing_mismatch == 0
            && self.vpos_mismatch == 0
    }
}

/// 문서 전체 비교 결과.
#[derive(Debug, Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinesegAgreement {
    pub rows: Vec<LinesegAgreementRow>,
    /// 저장 줄이 없어 비교하지 않은 문단 수
    pub no_stored_record: usize,
    /// 저장 줄 높이가 0 이라 한컴 확정값으로 볼 수 없어 뺀 문단 수
    pub invalid_stored: usize,
}

fn paragraph_height(segs: &[LineSeg]) -> i32 {
    match (segs.first(), segs.last()) {
        (Some(first), Some(last)) => {
            last.vertical_pos + last.line_height + last.line_spacing - first.vertical_pos
        }
        _ => 0,
    }
}

fn compare_row(
    scope: &'static str,
    section: usize,
    para: usize,
    path: String,
    depth: usize,
    paragraph: &Paragraph,
    paragraph_box: ParagraphBox,
    styles: &ResolvedStyleSet,
    dpi: f64,
    out: &mut LinesegAgreement,
) {
    let stored_len = paragraph
        .line_segs
        .len()
        .saturating_sub(paragraph.layout_only_fill_lines);
    let stored = &paragraph.line_segs[..stored_len];
    if stored.is_empty() {
        out.no_stored_record += 1;
        return;
    }
    if stored.iter().any(|seg| seg.line_height <= 0) {
        out.invalid_stored += 1;
        return;
    }
    let mut candidate = paragraph.clone();
    reflow_line_segs(&mut candidate, paragraph_box, styles, dpi);
    let reflowed = &candidate.line_segs[..candidate
        .line_segs
        .len()
        .saturating_sub(candidate.layout_only_fill_lines)];

    let mut row = LinesegAgreementRow {
        scope,
        section,
        para,
        path,
        depth,
        chars: paragraph.text.chars().count(),
        vertical_alignment: styles
            .para_styles
            .get(paragraph.para_shape_id as usize)
            .map_or(0, |style| style.vertical_alignment),
        stored_lines: stored.len(),
        reflowed_lines: reflowed.len(),
        break_mismatch: 0,
        line_height_mismatch: 0,
        text_height_mismatch: 0,
        baseline_mismatch: 0,
        line_spacing_mismatch: 0,
        vpos_mismatch: 0,
        max_line_height_delta: 0,
        max_baseline_delta: 0,
        max_vpos_delta: 0,
        stored_height: paragraph_height(stored),
        reflowed_height: paragraph_height(reflowed),
        first_diff: String::new(),
    };
    let brief = |seg: &LineSeg| {
        format!(
            "{}/{}/{}/{}/{}",
            seg.text_start,
            seg.line_height,
            seg.text_height,
            seg.baseline_distance,
            seg.line_spacing
        )
    };
    let stored_origin = stored[0].vertical_pos;
    let reflowed_origin = reflowed.first().map_or(0, |seg| seg.vertical_pos);
    for (line, (s, r)) in stored.iter().zip(reflowed).enumerate() {
        let lh = r.line_height - s.line_height;
        let bl = r.baseline_distance - s.baseline_distance;
        let vp = (r.vertical_pos - reflowed_origin) - (s.vertical_pos - stored_origin);
        row.break_mismatch += usize::from(r.text_start != s.text_start);
        row.line_height_mismatch += usize::from(lh != 0);
        row.text_height_mismatch += usize::from(r.text_height != s.text_height);
        row.baseline_mismatch += usize::from(bl != 0);
        row.line_spacing_mismatch += usize::from(r.line_spacing != s.line_spacing);
        row.vpos_mismatch += usize::from(vp != 0);
        if row.first_diff.is_empty()
            && (r.text_start != s.text_start
                || lh != 0
                || bl != 0
                || r.text_height != s.text_height
                || r.line_spacing != s.line_spacing)
        {
            row.first_diff = format!("{line}: {} => {}", brief(s), brief(r));
        }
        if lh.abs() > row.max_line_height_delta.abs() {
            row.max_line_height_delta = lh;
        }
        if bl.abs() > row.max_baseline_delta.abs() {
            row.max_baseline_delta = bl;
        }
        if vp.abs() > row.max_vpos_delta.abs() {
            row.max_vpos_delta = vp;
        }
    }
    out.rows.push(row);
}

/// 셀 문단에 편집 경로(`reflow_cell_paragraph_with_metrics`)와 같은 상자를 만든다.
fn cell_box(
    (cell_width, pad_left, pad_right): (i32, i16, i16),
    paragraph: &Paragraph,
    styles: &ResolvedStyleSet,
    dpi: f64,
) -> ParagraphBox {
    let available = (hwpunit_to_px(cell_width, dpi)
        - hwpunit_to_px(pad_left as i32, dpi)
        - hwpunit_to_px(pad_right as i32, dpi))
    .max(0.0);
    let style = styles.para_styles.get(paragraph.para_shape_id as usize);
    ParagraphBox::content_for_style(available, style, dpi)
}

/// `paragraphs` 안의 표 셀 문단을 (중첩 표까지) 차례로 넘긴다.
fn visit_cells(
    paragraphs: &mut [Paragraph],
    prefix: &str,
    depth: usize,
    styles: &ResolvedStyleSet,
    dpi: f64,
    f: &mut dyn FnMut(String, usize, &mut Paragraph, ParagraphBox),
) {
    for paragraph in paragraphs.iter_mut() {
        for (ctrl_idx, control) in paragraph.controls.iter_mut().enumerate() {
            let Control::Table(table) = &*control else {
                continue;
            };
            let metrics: Vec<_> = (0..table.cells.len())
                .map(|cell_idx| DocumentCore::cell_metrics_for_control(control, cell_idx))
                .collect();
            let line_wraps: Vec<u8> = (0..table.cells.len())
                .map(|cell_idx| DocumentCore::cell_line_wrap_for_control(control, cell_idx))
                .collect();
            let Control::Table(table) = control else {
                continue;
            };
            for (cell_idx, cell) in table.cells.iter_mut().enumerate() {
                let Some(metric) = metrics[cell_idx] else {
                    continue;
                };
                for (cell_para_idx, cell_para) in cell.paragraphs.iter_mut().enumerate() {
                    let path = format!("{prefix}{ctrl_idx}.{cell_idx}.{cell_para_idx}");
                    let paragraph_box = cell_box(metric, cell_para, styles, dpi)
                        .with_cell_line_wrap(line_wraps[cell_idx]);
                    f(path.clone(), depth, cell_para, paragraph_box);
                    visit_cells(
                        std::slice::from_mut(cell_para),
                        &format!("{path}>"),
                        depth + 1,
                        styles,
                        dpi,
                        f,
                    );
                }
            }
        }
    }
}

impl DocumentCore {
    /// 본문 문단에 편집 경로(`reflow_paragraph`)와 같은 상자를 만든다.
    fn body_box(&self, section_idx: usize, para_idx: usize) -> ParagraphBox {
        let section = &self.document.sections[section_idx];
        let column_def = Self::find_column_def_for_paragraph(&section.paragraphs, para_idx);
        let layout =
            PageLayoutInfo::from_page_def(&section.section_def.page_def, &column_def, self.dpi);
        let col_idx = self
            .para_column_map
            .get(section_idx)
            .and_then(|m| m.get(para_idx))
            .copied()
            .unwrap_or(0) as usize;
        let col_area = layout
            .column_areas
            .get(col_idx)
            .unwrap_or(&layout.column_areas[0]);
        let style = self
            .styles
            .para_styles
            .get(section.paragraphs[para_idx].para_shape_id as usize);
        ParagraphBox::body_for_style(col_area.width, style, self.dpi)
    }

    /// 모든 문단의 저장 LINE_SEG 와 편집 재조판 결과를 비교한다. 문서는 바뀌지 않는다.
    pub fn lineseg_agreement(&self) -> LinesegAgreement {
        let styles = self.resolve_render_styles();
        let dpi = self.dpi;
        let mut out = LinesegAgreement::default();
        for (section_idx, section) in self.document.sections.iter().enumerate() {
            for (para_idx, paragraph) in section.paragraphs.iter().enumerate() {
                let paragraph_box = self.body_box(section_idx, para_idx);
                compare_row(
                    "body",
                    section_idx,
                    para_idx,
                    String::new(),
                    0,
                    paragraph,
                    paragraph_box,
                    &styles,
                    dpi,
                    &mut out,
                );
                // 셀은 읽기만 하지만 방문기가 가변 참조를 받으므로 복제본으로 돈다.
                let mut host = paragraph.clone();
                visit_cells(
                    std::slice::from_mut(&mut host),
                    "",
                    1,
                    &styles,
                    dpi,
                    &mut |path, depth, cell_para, paragraph_box| {
                        compare_row(
                            "cell",
                            section_idx,
                            para_idx,
                            path,
                            depth,
                            cell_para,
                            paragraph_box,
                            &styles,
                            dpi,
                            &mut out,
                        );
                    },
                );
            }
        }
        out
    }

    /// 모든 문단(본문·셀)을 편집 경로로 다시 계산해 적용한 뒤의 쪽수.
    ///
    /// 문서 전체를 한 글자씩 고친 것과 같은 상태다. **문서를 바꾸므로** 측정 전용 사본에서만 쓴다.
    /// 반환값은 (적용 전 쪽수, 적용 후 쪽수).
    pub fn forced_reflow_page_count(&mut self) -> (u32, u32) {
        let before = self.page_count();
        let styles = self.resolve_render_styles();
        let dpi = self.dpi;
        for section_idx in 0..self.document.sections.len() {
            for para_idx in 0..self.document.sections[section_idx].paragraphs.len() {
                let paragraph_box = self.body_box(section_idx, para_idx);
                let paragraph = &mut self.document.sections[section_idx].paragraphs[para_idx];
                if !paragraph.line_segs.is_empty() {
                    reflow_line_segs(paragraph, paragraph_box, &styles, dpi);
                }
                visit_cells(
                    std::slice::from_mut(paragraph),
                    "",
                    1,
                    &styles,
                    dpi,
                    &mut |_, _, cell_para, paragraph_box| {
                        if !cell_para.line_segs.is_empty() {
                            reflow_line_segs(cell_para, paragraph_box, &styles, dpi);
                        }
                    },
                );
            }
            self.document.sections[section_idx].raw_stream = None;
            self.recompose_section(section_idx);
        }
        self.mark_all_sections_dirty();
        self.paginate();
        self.invalidate_page_tree_cache();
        (before, self.page_count())
    }
}
