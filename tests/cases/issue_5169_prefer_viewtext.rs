//! #5169/#7698: ViewText contains review history, not just visible content.
//!
//! The unchanged public HWP5 source has ten raw tables. Independent Hancom
//! Print (six pages) shows four; ordered TRACK_CHANGE hide metadata and ranges
//! select visible content before layout. The original history must still survive
//! an unedited save. Evidence: mydocs/pr/pr_7698_review_impl.md (Native/fresh
//! Docker WASM all-page minimum 91.69%, and identical original/saved Print).
#![cfg(not(target_arch = "wasm32"))]

use rhwp::model::control::Control;
use rhwp::model::document::Document;
use rhwp::model::paragraph::Paragraph;

const SAMPLE: &str = "samples/issue5169_viewtext_changetracking.hwp";

fn source() -> Vec<u8> {
    std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(SAMPLE))
        .unwrap_or_else(|e| panic!("read {SAMPLE}: {e}"))
}

fn count_tables(paras: &[Paragraph]) -> usize {
    paras
        .iter()
        .map(|p| {
            p.controls
                .iter()
                .map(|c| match c {
                    Control::Table(t) => {
                        1 + t
                            .cells
                            .iter()
                            .map(|cell| count_tables(&cell.paragraphs))
                            .sum::<usize>()
                    }
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn text(paras: &[Paragraph], result: &mut String) {
    for p in paras {
        result.push_str(&p.text);
        result.push('\n');
        for c in &p.controls {
            if let Control::Table(t) = c {
                for cell in &t.cells {
                    text(&cell.paragraphs, result);
                }
            }
        }
    }
}

fn visible_content(doc: &Document) -> String {
    let mut result = String::new();
    for section in &doc.sections {
        text(&section.paragraphs, &mut result);
    }
    result
}

fn assert_visible_content(doc: &Document) {
    assert_eq!(
        doc.sections
            .iter()
            .map(|s| count_tables(&s.paragraphs))
            .sum::<usize>(),
        4,
        "hidden review-history controls must not enter visible IR"
    );
    let content = visible_content(doc);
    assert!(content.contains("연구계획서"));
    assert!(content.contains("2018. 6"));
    assert!(
        !content.contains("연구제안계획서"),
        "partially hidden deletion"
    );
    assert!(
        !content.contains("무역구제제도분야 협상 관련"),
        "fully hidden paragraph and terminator"
    );
}

#[test]
fn viewtext_applies_saved_change_visibility_before_layout() {
    assert_visible_content(&rhwp::parser::parse_document(&source()).expect("parse"));
}

#[test]
fn unedited_save_preserves_review_stream_and_visible_content() {
    let before = rhwp::parser::parse_document(&source()).expect("parse");
    let saved = rhwp::serializer::serialize_document(&before).expect("save");
    let mut cfb = cfb::CompoundFile::open(std::io::Cursor::new(saved.clone())).expect("CFB");
    assert!(cfb.open_stream("/BodyText/Section0").is_ok());
    assert!(
        cfb.open_stream("/ViewText/Section0").is_ok(),
        "Hancom needs the review stream as ViewText"
    );
    let after = rhwp::parser::parse_document(&saved).expect("reopen");
    assert_visible_content(&after);
    assert_eq!(visible_content(&before), visible_content(&after));
    assert_eq!(before.sections.len(), after.sections.len());
    for (original, reopened) in before.sections.iter().zip(&after.sections) {
        assert!(original.raw_stream.is_some());
        assert_eq!(
            original.raw_stream, reopened.raw_stream,
            "raw review history is retained"
        );
    }
}

#[test]
fn unaffected_visible_paragraphs_keep_valid_saved_line_boundaries() {
    let doc = rhwp::parser::parse_document(&source()).expect("parse");
    // Original ViewText LineSeg starts independently match Hancom's Print
    // wrapping. Neither paragraph has hidden tracking ranges.
    for (prefix, starts) in [("따라서", [0, 39]), ("고위관리", [0, 56])] {
        let p = doc
            .sections
            .iter()
            .flat_map(|s| &s.paragraphs)
            .find(|p| {
                p.text
                    .trim_start_matches([' ', 'o'])
                    .trim_start()
                    .starts_with(prefix)
            })
            .unwrap_or_else(|| panic!("missing paragraph: {prefix}"));
        assert_eq!(
            p.line_segs
                .iter()
                .map(|line| line.text_start)
                .collect::<Vec<_>>(),
            starts
        );
    }
}
