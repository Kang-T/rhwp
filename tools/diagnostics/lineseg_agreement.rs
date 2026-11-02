//! 저장 LINE_SEG(한컴 확정값) 대비 편집 재조판 일치도 — 본문·표 셀 문단별 측정.
//!
//! rhwp 는 유효한 저장 LINE_SEG 를 그대로 그리므로 열기만 하면 한컴과 같아 보인다. 문단을
//! 편집하면 그 문단은 다시 계산되는데, 그 결과가 한컴 저장값과 다르면 줄 나눔·줄 높이·쪽수가
//! 달라진다. 이 도구는 모든 문단을 편집 경로와 같은 상자로 다시 계산해 저장값과 비교하고,
//! 차이를 유형별(줄 수·줄 바뀜 위치·줄 높이·글자 높이·기준선·줄간격·상대 vpos)로 센다.
//!
//! 사용:
//!     cargo run --profile release-test --example lineseg_agreement -- samples/a.hwp samples/hwpx
//!     # 문단별 TSV, 모든 문단을 재계산해 적용했을 때의 쪽수 변화
//!     cargo run --profile release-test --example lineseg_agreement -- samples -o output/ls.tsv --pages
//!
//! 대상: 인자로 준 파일과 디렉터리 재귀의 .hwp/.hwpx. 파일 단위 panic 은 격리해 기록한다.

use std::io::Write as _;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use rhwp::document_core::queries::lineseg_agreement::{LinesegAgreement, LinesegAgreementRow};
use rhwp::DocumentCore;

#[derive(Default, Clone, Copy)]
struct Tally {
    paragraphs: usize,
    all_match: usize,
    line_count: usize,
    breaks: usize,
    line_height: usize,
    text_height: usize,
    baseline: usize,
    line_spacing: usize,
    vpos: usize,
    height: usize,
}

impl Tally {
    fn add(&mut self, row: &LinesegAgreementRow) {
        self.paragraphs += 1;
        self.all_match += usize::from(row.all_match());
        self.line_count += usize::from(row.stored_lines != row.reflowed_lines);
        self.breaks += usize::from(row.break_mismatch > 0);
        self.line_height += usize::from(row.line_height_mismatch > 0);
        self.text_height += usize::from(row.text_height_mismatch > 0);
        self.baseline += usize::from(row.baseline_mismatch > 0);
        self.line_spacing += usize::from(row.line_spacing_mismatch > 0);
        self.vpos += usize::from(row.vpos_mismatch > 0);
        self.height += usize::from(row.stored_height != row.reflowed_height);
    }

    fn merge(&mut self, o: &Tally) {
        self.paragraphs += o.paragraphs;
        self.all_match += o.all_match;
        self.line_count += o.line_count;
        self.breaks += o.breaks;
        self.line_height += o.line_height;
        self.text_height += o.text_height;
        self.baseline += o.baseline;
        self.line_spacing += o.line_spacing;
        self.vpos += o.vpos;
        self.height += o.height;
    }

    fn cells(&self) -> String {
        let pct = |n: usize| {
            if self.paragraphs == 0 {
                "-".to_string()
            } else {
                format!("{n} ({:.1}%)", n as f64 * 100.0 / self.paragraphs as f64)
            }
        };
        format!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            self.paragraphs,
            pct(self.all_match),
            pct(self.line_count),
            pct(self.breaks),
            pct(self.line_height),
            pct(self.text_height),
            pct(self.baseline),
            pct(self.line_spacing),
            pct(self.vpos),
            pct(self.height),
        )
    }
}

struct FileResult {
    path: PathBuf,
    report: Option<LinesegAgreement>,
    pages: Option<(u32, u32)>,
    status: String,
}

fn collect(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_dir() {
        let mut entries: Vec<_> = std::fs::read_dir(path)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .collect();
        entries.sort();
        for entry in entries {
            collect(&entry, out);
        }
    } else if path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e.to_ascii_lowercase().as_str(), "hwp" | "hwpx"))
    {
        out.push(path.to_path_buf());
    }
}

fn measure(path: &Path, pages: bool) -> FileResult {
    let run = panic::catch_unwind(AssertUnwindSafe(|| {
        let bytes = std::fs::read(path).map_err(|e| format!("read_error:{e}"))?;
        let mut core =
            DocumentCore::from_bytes(&bytes).map_err(|e| format!("parse_error:{e:?}"))?;
        let report = core.lineseg_agreement();
        let pages = pages.then(|| core.forced_reflow_page_count());
        Ok::<_, String>((report, pages))
    }));
    match run {
        Ok(Ok((report, pages))) => FileResult {
            path: path.to_path_buf(),
            report: Some(report),
            pages,
            status: "ok".into(),
        },
        Ok(Err(status)) => FileResult {
            path: path.to_path_buf(),
            report: None,
            pages: None,
            status,
        },
        Err(_) => FileResult {
            path: path.to_path_buf(),
            report: None,
            pages: None,
            status: "panic".into(),
        },
    }
}

const HEADER: &str = "| 문단 | 완전 일치 | 줄 수 | 줄 바뀜 위치 | 줄 높이 | 글자 높이 | 기준선 | 줄간격 | 상대 vpos | 문단 높이 |";
const RULE: &str = "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|";

fn main() {
    let mut args = std::env::args().skip(1);
    let mut inputs = Vec::new();
    let mut tsv: Option<PathBuf> = None;
    let mut pages = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" => tsv = args.next().map(PathBuf::from),
            "--pages" => pages = true,
            _ => inputs.push(PathBuf::from(arg)),
        }
    }
    if inputs.is_empty() {
        eprintln!("사용: lineseg_agreement <파일|디렉터리>... [-o rows.tsv] [--pages]");
        std::process::exit(2);
    }
    let mut files = Vec::new();
    for input in &inputs {
        collect(input, &mut files);
    }

    // 진행 중 panic 메시지가 표를 덮지 않게 한다 — status 에 기록된다.
    panic::set_hook(Box::new(|_| {}));
    let results: Vec<FileResult> = files.iter().map(|f| measure(f, pages)).collect();
    let _ = panic::take_hook();

    let mut body_total = Tally::default();
    let mut cell_total = Tally::default();
    let (mut no_record, mut invalid) = (0usize, 0usize);
    let mut page_changed = 0usize;
    println!("## 파일별 (본문 / 셀: 비교 문단, 완전 일치, 쪽수 저장값→재계산)\n");
    println!("| 파일 | 본문 문단 | 본문 일치 | 셀 문단 | 셀 일치 | 쪽수 | 상태 |");
    println!("|---|---:|---:|---:|---:|---:|---|");
    for r in &results {
        let name = r
            .path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let Some(report) = &r.report else {
            println!("| {name} | | | | | | {} |", r.status);
            continue;
        };
        let (mut body, mut cell) = (Tally::default(), Tally::default());
        for row in &report.rows {
            if row.scope == "body" {
                body.add(row)
            } else {
                cell.add(row)
            }
        }
        body_total.merge(&body);
        cell_total.merge(&cell);
        no_record += report.no_stored_record;
        invalid += report.invalid_stored;
        let pct = |t: &Tally| {
            if t.paragraphs == 0 {
                "-".into()
            } else {
                format!("{:.1}%", t.all_match as f64 * 100.0 / t.paragraphs as f64)
            }
        };
        let page = match r.pages {
            Some((a, b)) => {
                page_changed += usize::from(a != b);
                format!("{a}→{b}")
            }
            None => "-".into(),
        };
        println!(
            "| {name} | {} | {} | {} | {} | {page} | ok |",
            body.paragraphs,
            pct(&body),
            cell.paragraphs,
            pct(&cell)
        );
    }

    println!("\n## 유형별 합계 (어긋난 문단 수)\n");
    println!("| 범위 {}", &HEADER[1..]);
    println!("|---{}", &RULE[1..]);
    println!("| 본문 | {} |", body_total.cells());
    println!("| 셀 | {} |", cell_total.cells());
    println!(
        "\n파일 {}개 (실패 {}개), 저장 줄 없음 {}문단, 저장 줄 높이 0 {}문단 제외.",
        results.len(),
        results.iter().filter(|r| r.report.is_none()).count(),
        no_record,
        invalid
    );
    if pages {
        println!("모든 문단을 재계산해 적용하면 쪽수가 바뀌는 파일: {page_changed}개");
    }

    if let Some(tsv) = tsv {
        if let Some(dir) = tsv.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let mut f = std::fs::File::create(&tsv).expect("TSV 파일을 만들 수 없습니다");
        writeln!(f, "file\tscope\tsection\tpara\tpath\tdepth\tchars\tvertical_alignment\tstored_lines\treflowed_lines\tbreak_mismatch\tline_height_mismatch\ttext_height_mismatch\tbaseline_mismatch\tline_spacing_mismatch\tvpos_mismatch\tmax_line_height_delta\tmax_baseline_delta\tmax_vpos_delta\tstored_height\treflowed_height\tfirst_diff").unwrap();
        for r in &results {
            let Some(report) = &r.report else { continue };
            for row in &report.rows {
                writeln!(
                    f,
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    r.path.display(), row.scope, row.section, row.para, row.path, row.depth, row.chars, row.vertical_alignment,
                    row.stored_lines, row.reflowed_lines, row.break_mismatch, row.line_height_mismatch,
                    row.text_height_mismatch, row.baseline_mismatch, row.line_spacing_mismatch,
                    row.vpos_mismatch, row.max_line_height_delta, row.max_baseline_delta,
                    row.max_vpos_delta, row.stored_height, row.reflowed_height, row.first_diff
                )
                .unwrap();
            }
        }
        eprintln!("문단별 TSV: {}", tsv.display());
    }
}
