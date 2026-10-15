//! Parser/IR contracts on explicitly synthetic PDFs, not visual fidelity claims.
#![cfg(feature = "pdf-import")]

use rhwp::model::{control::Control, provenance::SourceFormat};
use rhwp::parser::pdf::{parse_pdf_with_options, PdfImportOptions};
use std::collections::BTreeSet;

fn pdf(pages: &[(&str, &str)]) -> Vec<u8> {
    let mut objects = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
        String::new(),
    ];
    let mut kids = Vec::new();
    for (text, font) in pages {
        let page = objects.len() + 1;
        kids.push(format!("{page} 0 R"));
        let content = format!("BT /F1 12 Tf 30 140 Td ({text}) Tj ET");
        objects.push(format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << /Font << /F1 {} 0 R >> >> /Contents {} 0 R >>", page + 2, page + 1));
        objects.push(format!(
            "<< /Length {} >>\nstream\n{}\nendstream",
            content.len(),
            content
        ));
        objects.push(format!(
            "<< /Type /Font /Subtype /Type1 /BaseFont /{font} >>"
        ));
    }
    objects[1] = format!(
        "<< /Type /Pages /Count {} /Kids [{}] >>",
        pages.len(),
        kids.join(" ")
    );
    let mut output = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for (i, object) in objects.iter().enumerate() {
        offsets.push(output.len());
        output.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", i + 1, object).as_bytes());
    }
    let xref = output.len();
    output.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in offsets {
        output.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    output.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    output
}

fn texts(document: &rhwp::model::document::Document) -> Vec<String> {
    document
        .sections
        .iter()
        .map(|s| {
            s.paragraphs
                .iter()
                .flat_map(|p| &p.controls)
                .filter_map(|c| match c {
                    Control::Shape(s) => s.drawing().and_then(|d| d.text_box.as_ref()),
                    _ => None,
                })
                .flat_map(|b| &b.paragraphs)
                .map(|p| p.text.as_str())
                .collect()
        })
        .collect()
}

#[test]
fn all_pages_share_valid_style_pools_and_unique_instance_ids() {
    let source = pdf(&[("FIRST", "Helvetica"), ("SECOND", "Courier")]);
    let imported = parse_pdf_with_options(&source, &PdfImportOptions::default()).unwrap();
    let document = &imported.document;
    assert_eq!(texts(document), ["FIRST", "SECOND"]);
    assert_eq!(document.provenance.format, SourceFormat::Pdf);
    assert!(!document.layout_profile().native_hwp5_layout());
    assert!(!document.layout_profile().hwpx_container());
    assert_eq!(document.doc_properties.section_count, 2);
    let mut ids = BTreeSet::new();
    let mut fonts = Vec::new();
    for section in &document.sections {
        for control in &section.paragraphs[0].controls {
            let Control::Shape(shape) = control else {
                continue;
            };
            assert!(ids.insert(shape.common().instance_id));
            let tb = shape.drawing().unwrap().text_box.as_ref().unwrap();
            assert!(tb.text_margin_explicit);
            for paragraph in &tb.paragraphs {
                assert!(usize::from(paragraph.para_shape_id) < document.doc_info.para_shapes.len());
                for run in &paragraph.char_shapes {
                    let style = &document.doc_info.char_shapes[run.char_shape_id as usize];
                    fonts.push(
                        document.doc_info.font_faces[0][style.font_ids[0] as usize]
                            .name
                            .clone(),
                    );
                }
            }
        }
    }
    assert_eq!(fonts, ["Helvetica", "Courier"]);
    assert_eq!(
        imported
            .metadata
            .pages
            .iter()
            .map(|p| p.source_page)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    assert!(!imported.metadata.parser_allocation_bounds_proven);
}

#[test]
fn selection_is_explicit_and_keeps_source_page_mapping() {
    let source = pdf(&[("FIRST", "Helvetica"), ("SECOND", "Courier")]);
    let imported = parse_pdf_with_options(
        &source,
        &PdfImportOptions {
            source_pages: Some(vec![1]),
        },
    )
    .unwrap();
    assert_eq!(texts(&imported.document), ["SECOND"]);
    assert_eq!(imported.metadata.source_page_count, 2);
    assert_eq!(imported.metadata.pages[0].source_page, 1);
    assert_eq!(imported.metadata.pages[0].output_section, 0);
    for pages in [vec![], vec![2], vec![0, 0], vec![1, 0]] {
        let error = parse_pdf_with_options(
            &source,
            &PdfImportOptions {
                source_pages: Some(pages),
            },
        )
        .err()
        .unwrap();
        assert_eq!(error.code, "PDF_PAGE_SELECTION");
    }
}

#[test]
fn invalid_or_over_budget_input_does_not_poison_next_import() {
    for source in [
        b"not pdf".to_vec(),
        b"%PDF-1.4\ninvalid".to_vec(),
        vec![0; 16 * 1024 * 1024 + 1],
    ] {
        assert!(parse_pdf_with_options(&source, &PdfImportOptions::default()).is_err());
    }
    let imported =
        parse_pdf_with_options(&pdf(&[("OK", "Helvetica")]), &PdfImportOptions::default()).unwrap();
    assert_eq!(texts(&imported.document), ["OK"]);
}

#[test]
fn blank_pages_are_not_silently_dropped() {
    let imported = parse_pdf_with_options(
        &pdf(&[("", "Helvetica"), ("OK", "Helvetica")]),
        &PdfImportOptions::default(),
    )
    .unwrap();
    assert_eq!(imported.document.sections.len(), 2);
    assert_eq!(texts(&imported.document), ["", "OK"]);
    assert!(imported.metadata.pages[0].source_spans.is_empty());
}
