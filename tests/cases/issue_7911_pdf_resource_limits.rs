//! Synthetic resource contracts, not PDF/Hancom layout baselines.
#![cfg(feature = "pdf-import")]

use pdf_oxide::decoders::{decode_stream_with_budget, DecodeParams};
use pdf_oxide::error::Error;
use pdf_oxide::import_limits::{ImportBudget, PdfImportLimits};
use pdf_oxide::PdfDocument;
use std::io::Write;
use std::sync::Arc;

fn budget(stream: usize, total: usize) -> ImportBudget {
    ImportBudget::new(PdfImportLimits {
        stream_bytes: stream,
        decoded_bytes: total,
        ..PdfImportLimits::default()
    })
    .unwrap()
}

fn decode(data: &[u8], filters: &[&str], budget: &ImportBudget) -> Result<Vec<u8>, Error> {
    decode_stream_with_budget(
        data,
        &filters.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
        None,
        budget,
    )
}

fn flate(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(bytes).unwrap();
    encoder.finish().unwrap()
}

fn object_file(objects: &[Vec<u8>], trailer: &str) -> Vec<u8> {
    let mut bytes = b"%PDF-1.5\n".to_vec();
    let mut offsets = Vec::new();
    for (i, object) in objects.iter().enumerate() {
        offsets.push(bytes.len());
        write!(bytes, "{} 0 obj\n", i + 1).unwrap();
        bytes.extend(object);
        bytes.extend(b"\nendobj\n");
    }
    let xref = bytes.len();
    write!(
        bytes,
        "xref\n0 {}\n0000000000 65535 f \n",
        objects.len() + 1
    )
    .unwrap();
    for offset in offsets {
        writeln!(bytes, "{offset:010} 00000 n ").unwrap();
    }
    write!(
        bytes,
        "trailer\n<< /Size {} /Root 1 0 R {trailer} >>\nstartxref\n{xref}\n%%EOF\n",
        objects.len() + 1
    )
    .unwrap();
    bytes
}

fn objects(content: &[u8], filter: &str, catalog_extra: &str) -> Vec<Vec<u8>> {
    let mut stream = format!("<< /Length {} {filter} >>\nstream\n", content.len()).into_bytes();
    stream.extend(content);
    stream.extend(b"\nendstream");
    vec![
        format!("<< /Type /Catalog /Pages 2 0 R {catalog_extra} >>").into_bytes(),
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>".to_vec(),
        stream,
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
    ]
}

#[test]
fn individual_output_boundaries_are_exact_for_supported_filters() {
    let cases = [
        (b"4142".to_vec(), "ASCIIHexDecode", b"AB".to_vec()),
        (vec![1, b'A', b'B', 128], "RunLengthDecode", b"AB".to_vec()),
        (vec![128, 16, 72, 80, 16], "LZWDecode", b"AB".to_vec()),
        (flate(b"AB"), "FlateDecode", b"AB".to_vec()),
        (b"z~>".to_vec(), "ASCII85Decode", vec![0; 4]),
    ];
    for (encoded, filter, expected) in cases {
        for cap in [expected.len() - 1, expected.len(), expected.len() + 1] {
            let budget = budget(cap, 4096);
            let result = decode(&encoded, &[filter], &budget);
            if cap < expected.len() {
                assert!(
                    matches!(result, Err(Error::ImportResourceLimit(_))),
                    "{filter}: {result:?}"
                );
                assert!(budget.check().is_err());
            } else {
                assert_eq!(result.unwrap(), expected, "{filter}");
                assert_eq!(budget.decoded_bytes(), expected.len());
            }
        }
    }
}

#[test]
fn brotli_output_is_bounded_before_vec_extension() {
    // Independently encoded by Node zlib.brotliCompressSync(Buffer.alloc(128, 65)).
    let compressed = [27, 127, 0, 248, 37, 130, 34, 177, 64, 32, 3];
    for cap in [127, 128, 129] {
        let budget = budget(cap, 1024);
        let result = decode(&compressed, &["BrotliDecode"], &budget);
        if cap == 127 {
            assert!(matches!(result, Err(Error::ImportResourceLimit(_))));
            assert_eq!(budget.decoded_bytes(), 0);
        } else {
            assert_eq!(result.unwrap(), vec![65; 128]);
        }
    }
}

#[test]
fn cumulative_stream_budget_is_sticky_and_request_local() {
    let shared = budget(8, 7);
    assert_eq!(
        decode(b"41424344", &["ASCIIHexDecode"], &shared).unwrap(),
        b"ABCD"
    );
    assert!(matches!(
        decode(b"41424344", &["ASCIIHexDecode"], &shared),
        Err(Error::ImportResourceLimit(_))
    ));
    assert_eq!(shared.decoded_bytes(), 7);
    assert!(matches!(
        shared.reject("later failure"),
        Error::ImportResourceLimit("document decoded bytes")
    ));
    assert!(matches!(
        shared.check(),
        Err(Error::ImportResourceLimit("document decoded bytes"))
    ));
    assert!(decode(b"41", &["ASCIIHexDecode"], &shared).is_err());
    assert_eq!(
        decode(b"41", &["ASCIIHexDecode"], &budget(8, 8)).unwrap(),
        b"A"
    );
}

#[test]
fn intermediate_filter_outputs_consume_the_same_budget() {
    let shared = budget(8, 5);
    assert!(matches!(
        decode(b"34313432", &["ASCIIHexDecode", "ASCIIHexDecode"], &shared),
        Err(Error::ImportResourceLimit(_))
    ));
    assert_eq!(shared.decoded_bytes(), 5);
    assert_eq!(
        decode(
            b"34313432",
            &["ASCIIHexDecode", "ASCIIHexDecode"],
            &budget(8, 6)
        )
        .unwrap(),
        b"AB"
    );
}

#[test]
fn predictor_layout_overflow_and_zero_rows_are_rejected() {
    for columns in [0, usize::MAX] {
        let shared = budget(32, 128);
        let params = DecodeParams {
            predictor: 12,
            columns,
            colors: 3,
            bits_per_component: 8,
        };
        assert!(matches!(
            decode_stream_with_budget(&[0, 1, 2], &[], Some(&params), &shared),
            Err(Error::ImportResourceLimit(_))
        ));
    }
    let params = DecodeParams {
        predictor: 12,
        columns: 2,
        colors: 1,
        bits_per_component: 8,
    };
    assert_eq!(
        decode_stream_with_budget(&[0, 1, 2], &[], Some(&params), &budget(3, 16)).unwrap(),
        [1, 2]
    );
}

#[test]
fn zero_limits_do_not_disable_protection() {
    assert!(matches!(
        ImportBudget::new(PdfImportLimits {
            stream_bytes: 0,
            ..PdfImportLimits::default()
        }),
        Err(Error::ImportResourceLimit(_))
    ));
}

#[test]
fn lexical_depth_is_checked_before_the_recursive_parser_frame() {
    let policy = PdfImportLimits {
        nesting: 2,
        ..PdfImportLimits::default()
    };
    let accepted = Arc::new(ImportBudget::new(policy).unwrap());
    assert!(pdf_oxide::parser::parse_object_with_budget(b"[[0]]", Some(&accepted)).is_ok());
    let rejected = Arc::new(ImportBudget::new(policy).unwrap());
    assert!(pdf_oxide::parser::parse_object_with_budget(b"[[[0]]]", Some(&rejected)).is_err());
    assert!(rejected.check().is_err());
}

#[test]
fn parser_collection_and_object_counts_share_request_accounting() {
    for policy in [
        PdfImportLimits {
            entries: 1,
            ..PdfImportLimits::default()
        },
        PdfImportLimits {
            objects: 2,
            ..PdfImportLimits::default()
        },
    ] {
        let shared = Arc::new(ImportBudget::new(policy).unwrap());
        assert!(pdf_oxide::parser::parse_object_with_budget(b"[0 1]", Some(&shared)).is_err());
        assert!(shared.check().is_err());
    }
}

#[test]
fn cloned_streams_retain_the_original_document_budget() {
    let shared = Arc::new(budget(16, 11));
    let (_, object) = pdf_oxide::parser::parse_object_with_budget(
        b"<< /Length 4 >>\nstream\nABCD\nendstream",
        Some(&shared),
    )
    .unwrap();
    let cloned = object.clone();
    let pdf_oxide::object::Object::Stream { data, .. } = &cloned else {
        panic!("expected stream")
    };
    assert!(Arc::ptr_eq(data.budget().unwrap(), &shared));
    assert_eq!(object.decode_stream_data().unwrap(), b"ABCD");
    assert!(matches!(
        cloned.decode_stream_data(),
        Err(Error::ImportResourceLimit(_))
    ));
    assert_eq!(shared.decoded_bytes(), 8);
}

#[test]
fn declared_object_stream_count_is_checked_before_capacity() {
    let shared = Arc::new(ImportBudget::new(PdfImportLimits::default()).unwrap());
    let (_, object) = pdf_oxide::parser::parse_object_with_budget(
        b"<< /Type /ObjStm /N 100001 /First 0 /Length 1 >>\nstream\n0\nendstream",
        Some(&shared),
    )
    .unwrap();
    assert!(matches!(
        pdf_oxide::objstm::parse_object_stream(&object),
        Err(Error::ImportResourceLimit(_))
    ));
}

#[test]
fn traditional_xref_counts_use_the_constructor_budget() {
    let source = object_file(&objects(b"", "", ""), "");
    let limits = PdfImportLimits {
        entries: 2,
        ..PdfImportLimits::default()
    };
    assert!(matches!(
        PdfDocument::from_bytes_with_limits(source, limits),
        Err(Error::ImportResourceLimit(_))
    ));
}

#[test]
fn xref_stream_decode_uses_the_budget_before_document_creation() {
    let compressed = flate(&[0; 64]);
    let mut source = b"%PDF-1.5\n".to_vec();
    let offset = source.len();
    write!(source, "1 0 obj\n<< /Type /XRef /Size 1 /W [1 4 2] /Root 2 0 R /Length {} /Filter /FlateDecode >>\nstream\n", compressed.len()).unwrap();
    source.extend(compressed);
    write!(source, "\nendstream\nendobj\nstartxref\n{offset}\n%%EOF\n").unwrap();
    let limits = PdfImportLimits {
        stream_bytes: 32,
        ..PdfImportLimits::default()
    };
    assert!(matches!(
        PdfDocument::from_bytes_with_limits(source, limits),
        Err(Error::ImportResourceLimit(_))
    ));
}

#[test]
fn decode_limit_does_not_become_empty_paths_success() {
    let source = object_file(
        &objects(&flate(&[b'q'; 128]), "/Filter /FlateDecode", ""),
        "",
    );
    let doc = PdfDocument::from_bytes_with_limits(
        source,
        PdfImportLimits {
            stream_bytes: 64,
            ..PdfImportLimits::default()
        },
    )
    .unwrap();
    assert!(matches!(
        doc.extract_paths(0),
        Err(Error::ImportResourceLimit(_))
    ));
    assert!(doc.check_import_budget().is_err());
}

#[test]
fn content_operator_and_graphics_stack_limits_are_request_local() {
    let shared = Arc::new(
        ImportBudget::new(PdfImportLimits {
            operators: 2,
            graphics_stack: 1,
            ..PdfImportLimits::default()
        })
        .unwrap(),
    );
    let mut seen = 0;
    pdf_oxide::content::parser::execute_content_with_budget(b"q Q", &shared, |_| {
        seen += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(seen, 2);
    assert!(matches!(
        pdf_oxide::content::parser::execute_content_with_budget(b"q", &shared, |_| Ok(())),
        Err(Error::ImportResourceLimit(_))
    ));
    let next = Arc::new(
        ImportBudget::new(PdfImportLimits {
            graphics_stack: 1,
            ..PdfImportLimits::default()
        })
        .unwrap(),
    );
    assert!(matches!(
        pdf_oxide::content::parser::execute_content_with_budget(b"q q", &next, |_| Ok(())),
        Err(Error::ImportResourceLimit(_))
    ));
}

#[test]
fn content_operands_use_the_operator_domain_without_bypassing_limits() {
    let shared = Arc::new(
        ImportBudget::new(PdfImportLimits {
            objects: 1,
            ..PdfImportLimits::default()
        })
        .unwrap(),
    );
    let mut seen = 0;
    pdf_oxide::content::parser::execute_content_with_budget(
        b"1 0 0 1 10 20 cm 1 2 m 3 4 l S",
        &shared,
        |_| {
            seen += 1;
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seen, 4);
    assert!(pdf_oxide::parser::parse_object_with_budget(b"0", Some(&shared)).is_ok());
    assert!(pdf_oxide::parser::parse_object_with_budget(b"1", Some(&shared)).is_err());
    assert!(matches!(
        shared.check(),
        Err(Error::ImportResourceLimit("parsed object count"))
    ));

    for count in [64, 65] {
        let next = Arc::new(ImportBudget::new(PdfImportLimits::default()).unwrap());
        let content = format!("{}unknown", "0 ".repeat(count));
        let result = pdf_oxide::content::parser::execute_content_with_budget(
            content.as_bytes(),
            &next,
            |_| Ok(()),
        );
        if count == 64 {
            result.unwrap();
            next.check().unwrap();
        } else {
            assert!(matches!(result, Err(Error::ImportResourceLimit(_))));
            assert!(next.check().is_err());
        }
    }
}

#[test]
fn nested_content_operands_cannot_bypass_lexical_limits() {
    let shared = Arc::new(
        ImportBudget::new(PdfImportLimits {
            nesting: 2,
            ..PdfImportLimits::default()
        })
        .unwrap(),
    );
    assert!(matches!(
        pdf_oxide::content::parser::execute_content_with_budget(b"[[[0]]] TJ", &shared, |_| Ok(())),
        Err(Error::ImportResourceLimit(_))
    ));
}

#[test]
fn product_import_preserves_fatal_code_then_accepts_a_new_document() {
    use rhwp::parser::pdf::{parse_pdf_with_options, PdfImportOptions};
    let nested = format!("/Unused {}0{}", "[".repeat(32), "]".repeat(32));
    let source = object_file(&objects(b"", "", &nested), "");
    let error = parse_pdf_with_options(&source, &PdfImportOptions::default())
        .err()
        .unwrap();
    assert_eq!(error.code, "PDF_RESOURCE_LIMIT");
    let normal = object_file(&objects(b"BT /F1 12 Tf 20 100 Td (OK) Tj ET", "", ""), "");
    let imported = parse_pdf_with_options(&normal, &PdfImportOptions::default()).unwrap();
    assert_eq!(imported.document.sections.len(), 1);
    assert!(!imported.metadata.parser_allocation_bounds_proven);
}

#[test]
fn jpeg_header_mismatch_is_rejected_before_png_and_normal_pixels_are_unchanged() {
    use pdf_oxide::extractors::images::{ColorSpace, ImageData, PdfImage};
    let mut jpeg = Vec::new();
    image::codecs::jpeg::JpegEncoder::new(&mut jpeg)
        .encode(
            &[255, 0, 0, 0, 255, 0],
            2,
            1,
            image::ExtendedColorType::Rgb8,
        )
        .unwrap();
    let wrong = PdfImage::new(
        1,
        1,
        ColorSpace::DeviceRGB,
        8,
        ImageData::Jpeg(jpeg.clone()),
    );
    assert!(wrong.to_png_bytes_with_budget(&budget(4096, 8192)).is_err());
    let normal = PdfImage::new(2, 1, ColorSpace::DeviceRGB, 8, ImageData::Jpeg(jpeg));
    assert_eq!(
        normal
            .to_png_bytes_with_budget(&budget(4096, 8192))
            .unwrap(),
        normal.to_png_bytes().unwrap()
    );
}

#[test]
fn huge_image_declarations_are_rejected_without_allocating_pixels() {
    use pdf_oxide::extractors::images::{ColorSpace, ImageData, PdfImage, PixelFormat};
    let image = PdfImage::new(
        u32::MAX,
        u32::MAX,
        ColorSpace::DeviceRGB,
        8,
        ImageData::Raw {
            pixels: vec![],
            format: PixelFormat::RGB,
        },
    );
    assert!(matches!(
        image.to_png_bytes_with_budget(&budget(64, 128)),
        Err(Error::ImportResourceLimit(_))
    ));
}

#[test]
fn cmap_expansion_boundaries_apply_to_all_mapping_forms() {
    use pdf_oxide::fonts::cmap::parse_tounicode_cmap_with_budget;
    for (source, expected) in [
        (
            "beginbfrange\n<00> <02> <0041>\nendbfrange",
            ["A", "B", "C"],
        ),
        (
            "beginbfrange\n<00> <02> [<0041> <0042> <0043>]\nendbfrange",
            ["A", "B", "C"],
        ),
        (
            "beginnotdefrange\n<00> <02> <0041>\nendnotdefrange",
            ["A", "A", "A"],
        ),
    ] {
        for cap in [2, 3, 4] {
            let budget = ImportBudget::new(PdfImportLimits {
                cmap_entries: cap,
                ..PdfImportLimits::default()
            })
            .unwrap();
            let result = parse_tounicode_cmap_with_budget(source.as_bytes(), &budget);
            if cap == 2 {
                assert!(matches!(
                    result,
                    Err(Error::ImportResourceLimit("CMap entries"))
                ));
            } else {
                let cmap = result.unwrap();
                for (i, value) in expected.iter().enumerate() {
                    assert_eq!(cmap.get(&(i as u32)).as_deref(), Some(*value));
                }
            }
        }
    }
}

#[test]
fn cmap_declared_huge_ranges_and_reverse_ranges_do_not_wrap() {
    use pdf_oxide::fonts::cmap::parse_tounicode_cmap_with_budget;
    for source in [
        "beginbfrange\n<00000000> <FFFFFFFF> <0041>\nendbfrange",
        "beginbfrange\n<00000000> <FFFFFFFF> [<0041>]\nendbfrange",
        "beginnotdefrange\n<00000000> <FFFFFFFF> <0041>\nendnotdefrange",
    ] {
        let budget = ImportBudget::new(PdfImportLimits {
            cmap_entries: 16,
            ..PdfImportLimits::default()
        })
        .unwrap();
        assert!(matches!(
            parse_tounicode_cmap_with_budget(source.as_bytes(), &budget),
            Err(Error::ImportResourceLimit("CMap entries"))
        ));
    }
    let budget = ImportBudget::new(PdfImportLimits::default()).unwrap();
    let cmap =
        parse_tounicode_cmap_with_budget(b"beginbfrange\n<02> <00> [<0041>]\nendbfrange", &budget)
            .unwrap();
    assert!(cmap.is_empty());
    budget.check().unwrap();

    let budget = ImportBudget::new(PdfImportLimits {
        cmap_entries: 10_002,
        ..PdfImportLimits::default()
    })
    .unwrap();
    let cmap = parse_tounicode_cmap_with_budget(
        b"beginbfrange\n<0000> <2711> <0020>\nendbfrange",
        &budget,
    )
    .unwrap();
    let last = char::from_u32(0x20 + 10_001).unwrap().to_string();
    assert_eq!(cmap.get(&10_001).as_deref(), Some(last.as_str()));
}

#[test]
fn cmap_source_and_line_limits_precede_mapping_expansion() {
    use pdf_oxide::fonts::cmap::parse_tounicode_cmap_with_budget;
    let source = b"beginbfchar\n<00> <0041>\nendbfchar";
    for cap in [source.len() - 1, source.len(), source.len() + 1] {
        let budget = ImportBudget::new(PdfImportLimits {
            cmap_bytes: cap,
            ..PdfImportLimits::default()
        })
        .unwrap();
        let result = parse_tounicode_cmap_with_budget(source, &budget);
        if cap < source.len() {
            assert!(matches!(
                result,
                Err(Error::ImportResourceLimit("CMap source bytes"))
            ));
        } else {
            assert_eq!(result.unwrap().get(&0).as_deref(), Some("A"));
        }
    }
    let budget = ImportBudget::new(PdfImportLimits {
        string_bytes: 16,
        ..PdfImportLimits::default()
    })
    .unwrap();
    assert!(matches!(
        parse_tounicode_cmap_with_budget(b"xxxxxxxxxxxxxxxxx", &budget),
        Err(Error::ImportResourceLimit("CMap line bytes"))
    ));
    let source = b"beginbfchar beginbfchar beginbfchar";
    let budget = ImportBudget::new(PdfImportLimits {
        cmap_scan_bytes: source.len() * 5,
        ..PdfImportLimits::default()
    })
    .unwrap();
    assert!(matches!(
        parse_tounicode_cmap_with_budget(source, &budget),
        Err(Error::ImportResourceLimit("CMap scan bytes"))
    ));
}

#[test]
fn cmap_overwrites_and_repeated_parses_consume_request_work() {
    use pdf_oxide::fonts::cmap::parse_tounicode_cmap_with_budget;
    let source = b"beginbfchar\n<00> <0041>\n<00> <0042>\nendbfchar";
    let policy = PdfImportLimits {
        cmap_entries: 3,
        ..PdfImportLimits::default()
    };
    let budget = ImportBudget::new(policy).unwrap();
    assert_eq!(
        parse_tounicode_cmap_with_budget(source, &budget)
            .unwrap()
            .get(&0)
            .as_deref(),
        Some("B")
    );
    assert!(parse_tounicode_cmap_with_budget(source, &budget).is_err());
    assert!(parse_tounicode_cmap_with_budget(source, &ImportBudget::new(policy).unwrap()).is_ok());
}

#[test]
fn limited_lazy_cmap_bypasses_global_cache_and_shares_clones() {
    use pdf_oxide::fonts::cmap::LazyCMap;
    let source = b"beginbfchar\n<11> <0051>\n<12> <0052>\n<13> <0053>\nendbfchar";
    let legacy = LazyCMap::new(source.to_vec());
    assert_eq!(legacy.get().unwrap().get(&0x11).as_deref(), Some("Q"));
    let low = Arc::new(
        ImportBudget::new(PdfImportLimits {
            cmap_entries: 2,
            ..PdfImportLimits::default()
        })
        .unwrap(),
    );
    let limited = LazyCMap::new_with_budget(source.to_vec(), Arc::clone(&low)).unwrap();
    assert!(limited.get().is_none());
    assert!(low.check().is_err());

    let fresh = Arc::new(
        ImportBudget::new(PdfImportLimits {
            cmap_entries: 3,
            ..PdfImportLimits::default()
        })
        .unwrap(),
    );
    let lazy = LazyCMap::new_with_budget(source.to_vec(), Arc::clone(&fresh)).unwrap();
    let cloned = lazy.clone();
    assert_eq!(lazy.raw_data().as_ptr(), cloned.raw_data().as_ptr());
    assert!(Arc::ptr_eq(&lazy.get().unwrap(), &cloned.get().unwrap()));
    fresh.reject("test request exhausted");
    assert!(cloned.get().is_none());
}

#[test]
fn malformed_non_ascii_cmap_hex_cannot_panic_at_utf8_boundaries() {
    use pdf_oxide::fonts::cmap::parse_tounicode_cmap_with_budget;
    for source in [
        b"beginbfchar\n<01> <AAA\x80AAAAA>\nendbfchar".as_slice(),
        b"beginbfrange\n<01> <01> [<AAA\x80AAAAA>]\nendbfrange".as_slice(),
    ] {
        let budget = ImportBudget::new(PdfImportLimits::default()).unwrap();
        assert!(parse_tounicode_cmap_with_budget(source, &budget)
            .unwrap()
            .is_empty());
        assert!(pdf_oxide::fonts::cmap::parse_tounicode_cmap(source)
            .unwrap()
            .is_empty());
    }
}

#[test]
fn cid_metric_expansion_is_bounded_before_range_or_array_materialization() {
    use pdf_oxide::fonts::font_dict::FontInfo;
    for metrics in [
        "/W [0 2 500]",
        "/W [0 [500 500 500]]",
        "/W2 [0 2 -1000 500 880]",
        "/W2 [0 [-1000 500 880 -1000 500 880 -1000 500 880]]",
    ] {
        let font = format!("<< /Type /Font /Subtype /Type0 /BaseFont /Probe /Encoding /Identity-H /DescendantFonts [<< /Type /Font /Subtype /CIDFontType2 /BaseFont /Probe /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> {metrics} >>] >>");
        let (_, font) = pdf_oxide::parser::parse_object(font.as_bytes()).unwrap();
        for cap in [2, 3, 4] {
            let doc = PdfDocument::from_bytes_with_limits(
                object_file(&objects(b"", "", ""), ""),
                PdfImportLimits {
                    font_entries: cap,
                    ..PdfImportLimits::default()
                },
            )
            .unwrap();
            let result = FontInfo::from_dict(&font, &doc);
            if cap == 2 {
                assert!(matches!(
                    doc.check_import_budget(),
                    Err(Error::ImportResourceLimit("font metric entries"))
                ));
                assert!(result.is_err());
            } else {
                assert!(result.is_ok(), "{metrics}: {result:?}");
                doc.check_import_budget().unwrap();
            }
        }
    }
}

#[test]
fn content_array_combination_includes_separator_bytes_in_the_buffer_cap() {
    let mut parts = objects(b"1234", "", "");
    parts[2] =
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Contents [4 0 R 6 0 R] >>".to_vec();
    parts.push(b"<< /Length 4 >>\nstream\n5678\nendstream".to_vec());
    for cap in [9, 10, 11] {
        let doc = PdfDocument::from_bytes_with_limits(
            object_file(&parts, ""),
            PdfImportLimits {
                stream_bytes: cap,
                ..PdfImportLimits::default()
            },
        )
        .unwrap();
        let result = doc.get_page_content_data(0);
        if cap == 9 {
            assert!(matches!(
                result,
                Err(Error::ImportResourceLimit("stream output bytes"))
            ));
            assert!(doc.get_page_content_data(0).is_err());
        } else {
            assert_eq!(result.unwrap(), b"1234\n5678\n");
        }
    }
}

#[test]
fn cached_content_copies_consume_budget_and_cannot_revive_failed_imports() {
    let source = object_file(&objects(b"1234", "", ""), "");
    let doc = PdfDocument::from_bytes_with_limits(
        source.clone(),
        PdfImportLimits {
            stream_bytes: 8,
            decoded_bytes: 16,
            ..PdfImportLimits::default()
        },
    )
    .unwrap();
    assert_eq!(doc.get_page_content_data(0).unwrap(), b"1234");
    assert_eq!(doc.get_page_content_data(0).unwrap(), b"1234");
    assert!(matches!(
        doc.get_page_content_data(0),
        Err(Error::ImportResourceLimit("document decoded bytes"))
    ));
    assert!(doc.get_page_content_data(0).is_err());
    let next = PdfDocument::from_bytes_with_limits(source, PdfImportLimits::default()).unwrap();
    assert_eq!(next.get_page_content_data(0).unwrap(), b"1234");
}

#[test]
fn product_import_does_not_swallow_lazy_cmap_resource_failure() {
    use rhwp::parser::pdf::{parse_pdf_with_options, PdfImportOptions};
    let mut parts = objects(b"BT /F1 12 Tf 20 100 Td (A) Tj ET", "", "");
    parts[4] = b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /ToUnicode 6 0 R >>".to_vec();
    let cmap = b"beginbfrange\n<00000000> <FFFFFFFF> <0041>\nendbfrange";
    let mut stream = format!("<< /Length {} >>\nstream\n", cmap.len()).into_bytes();
    stream.extend(cmap);
    stream.extend(b"\nendstream");
    parts.push(stream);
    let error = parse_pdf_with_options(&object_file(&parts, ""), &PdfImportOptions::default())
        .err()
        .unwrap();
    assert_eq!(error.code, "PDF_RESOURCE_LIMIT");
    let normal = object_file(&objects(b"BT /F1 12 Tf 20 100 Td (OK) Tj ET", "", ""), "");
    assert!(parse_pdf_with_options(&normal, &PdfImportOptions::default()).is_ok());
}

fn page_tree_file(depth: usize, count: i64) -> Vec<u8> {
    let mut parts = vec![b"<< /Type /Catalog /Pages 2 0 R >>".to_vec()];
    for level in 0..depth {
        parts.push(
            format!(
                "<< /Type /Pages /Count {count} /Kids [{} 0 R] >>",
                level + 3
            )
            .into_bytes(),
        );
    }
    parts.push(b"<< /Type /Page /MediaBox [0 0 200 200] >>".to_vec());
    object_file(&parts, "")
}

fn form_tree_file(depth: usize) -> Vec<u8> {
    let mut parts = objects(b"/Fm Do", "", "");
    parts[2] = b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << /XObject << /Fm 6 0 R >> >> /Contents 4 0 R >>".to_vec();
    for level in 0..depth {
        let (content, xobjects) = if level + 1 == depth {
            ("BT /F1 12 Tf 20 100 Td (VISIBLE) Tj ET", String::new())
        } else {
            (
                "BT ET /Fm Do",
                format!("/XObject << /Fm {} 0 R >>", level + 7),
            )
        };
        parts.push(format!("<< /Type /XObject /Subtype /Form /BBox [0 0 200 200] /Resources << /Font << /F1 5 0 R >> {xobjects} >> /Length {} >>\nstream\n{content}\nendstream", content.len()).into_bytes());
    }
    object_file(&parts, "")
}

#[test]
fn traversal_page_depth_limit_reaches_public_page_count_and_get_page() {
    for depth in [31, 32, 33] {
        let doc = PdfDocument::from_bytes_with_limits(
            page_tree_file(depth, 1),
            PdfImportLimits::default(),
        )
        .unwrap();
        let result = doc.get_page(0);
        if depth > 32 {
            assert!(matches!(result, Err(Error::ImportResourceLimit(_))));
            assert!(matches!(
                doc.page_count(),
                Err(Error::ImportResourceLimit(_))
            ));
        } else {
            result.unwrap();
            assert_eq!(doc.page_count().unwrap(), 1);
        }
    }
}

#[test]
fn traversal_page_count_scan_cannot_return_partial_success_after_depth_limit() {
    let doc = PdfDocument::from_bytes_with_limits(
        page_tree_file(33, 8_388_608),
        PdfImportLimits::default(),
    )
    .unwrap();
    assert!(matches!(
        doc.page_count(),
        Err(Error::ImportResourceLimit(_))
    ));
    assert!(doc.get_page(0).is_err());
}

#[test]
fn traversal_failed_request_cannot_return_a_cached_page_or_object() {
    let doc = PdfDocument::from_bytes_with_limits(page_tree_file(1, 1), PdfImportLimits::default())
        .unwrap();
    let reference = pdf_oxide::object::ObjectRef::new(2, 0);
    doc.get_page(0).unwrap();
    doc.load_object(reference).unwrap();
    doc.import_budget()
        .unwrap()
        .reject("test traversal failure");
    assert!(matches!(
        doc.get_page(0),
        Err(Error::ImportResourceLimit("test traversal failure"))
    ));
    assert!(matches!(
        doc.load_object(reference),
        Err(Error::ImportResourceLimit("test traversal failure"))
    ));
    let fresh =
        PdfDocument::from_bytes_with_limits(page_tree_file(1, 1), PdfImportLimits::default())
            .unwrap();
    assert_eq!(fresh.page_count().unwrap(), 1);
    fresh.get_page(0).unwrap();
}

#[test]
fn traversal_form_depth_limit_is_fatal_instead_of_silent_text_loss() {
    for depth in [15, 16, 17] {
        let doc =
            PdfDocument::from_bytes_with_limits(form_tree_file(depth), PdfImportLimits::default())
                .unwrap();
        let text = doc.extract_text(0);
        if depth > 16 {
            assert!(matches!(text, Err(Error::ImportResourceLimit(_))));
            assert!(doc.get_page(0).is_err());
        } else {
            assert!(text.unwrap().contains("VISIBLE"));
        }
    }
}

#[test]
fn traversal_product_import_propagates_tree_and_form_failures_then_recovers() {
    use rhwp::parser::pdf::{parse_pdf_with_options, PdfImportOptions};
    for input in [page_tree_file(33, 1), form_tree_file(17)] {
        let error = parse_pdf_with_options(&input, &PdfImportOptions::default())
            .err()
            .unwrap();
        assert_eq!(error.code, "PDF_RESOURCE_LIMIT");
        assert!(
            parse_pdf_with_options(&page_tree_file(1, 1), &PdfImportOptions::default()).is_ok()
        );
    }
}

#[test]
fn traversal_form_depth_and_work_limits_cover_each_product_extraction_mode() {
    for mode in ["text", "paths", "images"] {
        for (depth, cap) in [(2, 3), (3, 3), (4, 3)] {
            let doc = PdfDocument::from_bytes_with_limits(
                form_tree_file(depth),
                PdfImportLimits {
                    form_depth: cap,
                    ..PdfImportLimits::default()
                },
            )
            .unwrap();
            let result = match mode {
                "text" => doc.extract_spans(0).map(|spans| {
                    assert!(spans.iter().any(|s| s.text.contains("VISIBLE")));
                }),
                "paths" => doc.extract_paths(0).map(|paths| assert!(paths.is_empty())),
                _ => doc
                    .page_image_handles(0)
                    .map(|images| assert!(images.is_empty())),
            };
            if depth > cap {
                assert!(
                    matches!(result, Err(Error::ImportResourceLimit("Form depth"))),
                    "{mode}: {result:?}"
                );
            } else {
                result.unwrap();
                doc.check_import_budget().unwrap();
            }
        }
        // Span postprocessing stamps character origins using a second content
        // interpretation. Both passes must consume the same request work budget.
        let visits = if mode == "text" { 6 } else { 3 };
        for cap in [visits - 1, visits, visits + 1] {
            let doc = PdfDocument::from_bytes_with_limits(
                form_tree_file(3),
                PdfImportLimits {
                    xobject_visits: cap,
                    ..PdfImportLimits::default()
                },
            )
            .unwrap();
            let result = match mode {
                "text" => doc.extract_spans(0).map(|_| ()),
                "paths" => doc.extract_paths(0).map(|_| ()),
                _ => doc.page_image_handles(0).map(|_| ()),
            };
            if cap < visits {
                assert!(
                    matches!(result, Err(Error::ImportResourceLimit("XObject visits"))),
                    "{mode}: {result:?}"
                );
                assert!(doc.get_page(0).is_err());
            } else {
                result.unwrap_or_else(|error| panic!("{mode}, visit cap {cap}: {error:?}"));
            }
        }
    }
}

#[test]
fn traversal_object_cache_hits_still_consume_request_visits() {
    let doc = PdfDocument::from_bytes_with_limits(
        page_tree_file(1, 1),
        PdfImportLimits {
            reference_visits: 64,
            ..PdfImportLimits::default()
        },
    )
    .unwrap();
    let reference = pdf_oxide::object::ObjectRef::new(2, 0);
    doc.load_object(reference).unwrap();
    let mut rejected = false;
    for _ in 0..65 {
        match doc.load_object(reference) {
            Ok(_) => {}
            Err(Error::ImportResourceLimit("reference visits")) => {
                rejected = true;
                break;
            }
            Err(error) => panic!("unexpected failure: {error:?}"),
        }
    }
    assert!(rejected);
    assert!(doc.get_page(0).is_err());
}

#[test]
fn traversal_bulk_page_cache_cannot_swallow_deep_branch_failure() {
    let mut kids = (3..67).map(|id| format!("{id} 0 R")).collect::<Vec<_>>();
    kids.push("67 0 R".into());
    let mut parts = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        format!("<< /Type /Pages /Count 65 /Kids [{}] >>", kids.join(" ")).into_bytes(),
    ];
    for _ in 0..64 {
        parts.push(b"<< /Type /Page /MediaBox [0 0 200 200] >>".to_vec());
    }
    for id in 67..99 {
        parts.push(format!("<< /Type /Pages /Count 1 /Kids [{} 0 R] >>", id + 1).into_bytes());
    }
    parts.push(b"<< /Type /Page /MediaBox [0 0 200 200] >>".to_vec());
    let doc =
        PdfDocument::from_bytes_with_limits(object_file(&parts, ""), PdfImportLimits::default())
            .unwrap();
    for page in 0..64 {
        doc.get_page(page).unwrap();
    }
    assert!(matches!(
        doc.get_page(64),
        Err(Error::ImportResourceLimit("page tree depth"))
    ));
    assert!(doc.get_page(0).is_err());
}

#[test]
fn traversal_page_count_uses_actual_leaves_instead_of_declared_count() {
    let parts = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Count 1 /Kids [3 0 R 4 0 R] >>".to_vec(),
        b"<< /Type /Page /MediaBox [0 0 200 200] >>".to_vec(),
        b"<< /Type /Page /MediaBox [0 0 200 200] >>".to_vec(),
    ];
    let doc =
        PdfDocument::from_bytes_with_limits(object_file(&parts, ""), PdfImportLimits::default())
            .unwrap();
    assert_eq!(doc.page_count().unwrap(), 2);
    doc.get_page(1).unwrap();
}

#[test]
fn traversal_zero_policy_values_cannot_disable_protection() {
    for limits in [
        PdfImportLimits {
            reference_depth: 0,
            ..PdfImportLimits::default()
        },
        PdfImportLimits {
            reference_visits: 0,
            ..PdfImportLimits::default()
        },
        PdfImportLimits {
            form_depth: 0,
            ..PdfImportLimits::default()
        },
        PdfImportLimits {
            xobject_visits: 0,
            ..PdfImportLimits::default()
        },
    ] {
        assert!(matches!(
            ImportBudget::new(limits),
            Err(Error::ImportResourceLimit("zero import budget"))
        ));
    }
}
