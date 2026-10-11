//! 일반 행·rowspan 행의 요구 높이와 잔여 밴드 Query. 예산 수용과 상태 반영은 부모 소유다.

use super::RowBlockQuery;
use crate::model::control::Control;
use crate::renderer::layout::table_layout::RowCutResult;
use crate::renderer::typeset::MIN_TOP_KEEP_PX;

pub(in crate::renderer::typeset) struct RowScanQuery<'a> {
    pub(in crate::renderer::typeset) rows: &'a RowBlockQuery<'a>,
    pub(in crate::renderer::typeset) r: usize,
    pub(in crate::renderer::typeset) cursor_row: usize,
    pub(in crate::renderer::typeset) start_cut: &'a [usize],
    pub(in crate::renderer::typeset) start_row_height_override: Option<f64>,
    pub(in crate::renderer::typeset) rowspan_physical_space: &'a [(usize, f64)],
    pub(in crate::renderer::typeset) accepted_rows: &'a [(usize, f64)],
}

pub(in crate::renderer::typeset) struct RowBandShape {
    pub(in crate::renderer::typeset) has_prior_rowspan_cover: bool,
    pub(in crate::renderer::typeset) row_has_nested: bool,
}

pub(in crate::renderer::typeset) struct RowBandProbe {
    pub(in crate::renderer::typeset) probe: RowCutResult,
    pub(in crate::renderer::typeset) visible_height: f64,
}

impl RowScanQuery<'_> {
    /// 앞 행이 실제 수용한 높이를 차감한다. 원본 측정 행과 컷용 행 높이를 합치지 않는다.
    pub(in crate::renderer::typeset) fn required_height(
        &self,
        height: f64,
        consumed: f64,
        cs_before: f64,
    ) -> f64 {
        let Self {
            rows,
            r,
            cursor_row,
            start_cut,
            start_row_height_override,
            rowspan_physical_space,
            accepted_rows,
        } = *self;
        let RowBlockQuery {
            layout_engine,
            mt,
            table,
            styles,
            ..
        } = *rows;
        // 시작 컷이 있는 행은 이미 소비한 내용을 다시 예약하지 않는다.
        // 완전 행의 선언 높이는 병합 최소에 분배되어 있으므로 기존처럼
        // 내용 하한만 사용하고, 병합 원장으로 남은 물리 높이를 정한다.
        let cut_minimum_height = (r == cursor_row && !start_cut.is_empty()).then_some(height);
        let height = layout_engine
            .rowspan_remaining_physical_row_height(
                table,
                r,
                cursor_row,
                rowspan_physical_space,
                accepted_rows,
                styles,
                cut_minimum_height,
            )
            .unwrap_or(height);
        layout_engine
            .straddle_continuation_demand(
                table,
                r,
                cursor_row,
                start_cut,
                start_row_height_override,
                &mt.row_heights,
                styles,
                (r + 1, true),
                !rowspan_physical_space.is_empty(),
            )
            .map_or(height, |need| height.max(need - consumed - cs_before))
    }

    pub(in crate::renderer::typeset) fn whole_row_height(
        &self,
        row_start_cut: &[usize],
        whole_row_fit_h: &[f64],
    ) -> f64 {
        let Self { rows, r, .. } = *self;
        let RowBlockQuery {
            layout_engine,
            table,
            styles,
            ..
        } = *rows;
        if row_start_cut.is_empty() {
            whole_row_fit_h[r]
        } else {
            // 연속분 cursor_row — 시작 컷 적용. row_cut_content_height 가
            // 셀별 (content+pad) 행 max 를 반환(분할 행이므로 cell.height
            // 강제 없음).
            let content =
                layout_engine.row_cut_content_height(table, r, row_start_cut, &[], styles);
            // The continuation cursor carries space already accepted by the
            // previous fragment, independently of its consumed content units.
            self.start_row_height_override
                .map_or(content, |physical| content.max(physical))
        }
    }

    pub(in crate::renderer::typeset) fn band_shape(&self) -> RowBandShape {
        let Self { rows, r, .. } = *self;
        let table = rows.table;
        let has_prior_rowspan_cover = table.cells.iter().any(|c| {
            c.row_span > 1 && (c.row as usize) < r && r < c.row as usize + c.row_span as usize
        });
        let row_has_nested = table.cells.iter().any(|c| {
            c.row as usize == r
                && c.paragraphs.iter().any(|p| {
                    p.controls
                        .iter()
                        .any(|ctrl| matches!(ctrl, Control::Table(_)))
                })
        });

        RowBandShape {
            has_prior_rowspan_cover,
            row_has_nested,
        }
    }

    /// 첫 저장 프레임 끝에서 덮는 모든 셀의 내용이 완전히 들어가는지 확인한다.
    /// 선언 셀 높이 대신 이번 조각이 실제 수용한 행 밴드와 같은 안 여백을 쓴다.
    /// 병합 상자의 물리 범위가 다음 행까지 이어져도 내용은 먼저 끝날 수 있다.
    /// 다음 조각은 실제 수용한 밴드를 이어받으므로 빈 상자와 내용을 다시 소비하지 않는다.
    /// 앞 조각에서 시작한 셀이나 개체/편집 내용에는 이 경로를 적용하지 않는다.
    pub(in crate::renderer::typeset) fn completed_frame_band_fits(&self, rest: f64) -> bool {
        let Self {
            rows,
            r,
            cursor_row,
            accepted_rows,
            ..
        } = *self;
        let RowBlockQuery {
            layout_engine,
            table,
            styles,
            cs,
            ..
        } = *rows;
        if layout_engine.row_uses_reflow_physical_frame(table, r) {
            return false;
        }
        table
            .cells
            .iter()
            .filter(|cell| cell.row as usize <= r && r < cell.row as usize + cell.row_span as usize)
            .all(|cell| {
                let begin = cell.row as usize;
                if begin < cursor_row
                    || cell.paragraphs.iter().any(|para| {
                        para.stored_text_partition_is_dirty() || !para.controls.is_empty()
                    })
                {
                    return false;
                }
                let mut band = rest;
                for row in begin..r {
                    let Some((_, height)) = accepted_rows.iter().find(|(owner, _)| *owner == row)
                    else {
                        return false;
                    };
                    band += height + cs;
                }
                let (_, _, top, bottom) =
                    layout_engine.resolve_cell_padding_for_physical_height(cell, table, band);
                layout_engine.cell_units_content_height(cell, table, styles) + top + bottom
                    <= band + 0.5
            })
    }

    /// 부모의 기존 guard 안에서만 실행한다. 컷 조회와 실제 표시 높이 조회 순서를 유지한다.
    pub(in crate::renderer::typeset) fn probe_band(
        &self,
        row_start_cut: &[usize],
        rest: f64,
    ) -> RowBandProbe {
        let Self { rows, r, .. } = *self;
        let RowBlockQuery {
            layout_engine,
            table,
            styles,
            ..
        } = *rows;
        let (probe, _) =
            layout_engine.advance_row_cut_for_physical_band(table, r, row_start_cut, rest, styles);
        let visible_height =
            layout_engine.row_cut_content_height(table, r, row_start_cut, &probe.end_cut, styles);

        RowBandProbe {
            probe,
            visible_height,
        }
    }
}

pub(in crate::renderer::typeset) fn retains_blank_tail(
    probe: &RowCutResult,
    visible_height: f64,
    rest: f64,
) -> bool {
    let retained_blank_tail = (rest - visible_height).max(0.0);
    probe.fully_consumed && visible_height <= rest + 0.5 && retained_blank_tail >= MIN_TOP_KEEP_PX
}
