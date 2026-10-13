//! Materialize geometry-confirmed grids as editable rhwp tables. Uncertain grids stay positional.
use crate::{lowering::hu, ExtractedPage};
use rhwp::model::{
    control::Control,
    document::Document,
    paragraph::{CharShapeRef, LineSeg, Paragraph},
    shape::{CommonObjAttr, HorzRelTo, ShapeObject, TextWrap, VertRelTo},
    style::{border_width_index, Alignment, LineSpacingType, PARA_ATTR2_LINE_WRAP_MASK},
    table::{Cell, Table, TablePageBreak, VerticalAlign},
};
use serde_json::{json, Value};
use std::collections::BTreeSet;

fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array()
        .ok_or_else(|| "invalid recognition array".into())
}
fn number(v: &Value) -> Result<f32, String> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .map(|n| n as f32)
        .ok_or_else(|| "invalid recognition coordinate".into())
}
fn index(v: &Value) -> Result<usize, String> {
    v.as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| "invalid recognition index".into())
}
fn refs(v: &Value) -> Result<Vec<usize>, String> {
    array(v)?.iter().map(index).collect()
}

fn text_with_spaces(page: &ExtractedPage, indices: &[usize]) -> (String, Vec<(u32, usize)>) {
    let mut text = String::new();
    let mut runs = Vec::new();
    let mut end = None;
    for &i in indices {
        let span = &page.spans[i];
        if end.is_some_and(|x| span.bbox.x - x > span.font_size * 0.2)
            && !text.ends_with(char::is_whitespace)
            && !span.text.starts_with(char::is_whitespace)
        {
            text.push(' ');
        }
        runs.push((text.encode_utf16().count() as u32, i));
        for (j, ch) in span.text.chars().enumerate() {
            if j > 0 {
                if let (Some(x), Some(prev), Some(width)) = (
                    span.char_x_offsets.get(j),
                    span.char_x_offsets.get(j - 1),
                    span.char_widths.get(j - 1),
                ) {
                    if x - prev - width > span.font_size * 0.2
                        && !text.ends_with(char::is_whitespace)
                        && !ch.is_whitespace()
                    {
                        text.push(' ');
                    }
                }
            }
            text.push(ch);
        }
        end = Some(span.bbox.x + span.bbox.width);
    }
    (text, runs)
}

fn offsets(text: &str) -> (Vec<u32>, u32) {
    let mut len = 0;
    let offsets = text
        .chars()
        .map(|c| {
            let at = len;
            len += c.len_utf16() as u32;
            at
        })
        .collect();
    (offsets, len)
}

struct FlowItem {
    top: f32,
    bottom: f32,
    paragraph: Paragraph,
}

fn paragraph(
    page: &ExtractedPage,
    doc: &mut Document,
    proposal: &Value,
    cell_box: &Value,
    shape_ids: &[Option<u32>],
) -> Result<FlowItem, String> {
    let lines = array(&proposal["lines"])?;
    let first = lines.first().ok_or("paragraph has no lines")?;
    let cell_x = number(&cell_box["x"])?;
    let cell_width = number(&cell_box["width"])?;
    let top = number(&first["baseline"])? - number(&first["fontSize"])? * 0.95;
    let centered = lines.iter().all(|l| {
        let b = &l["bbox"];
        let mid =
            b["x"].as_f64().unwrap_or(f64::NAN) + b["width"].as_f64().unwrap_or(f64::NAN) / 2.;
        (mid as f32 - cell_x - cell_width / 2.).abs() <= number(&l["fontSize"]).unwrap_or(0.) * 0.75
    });
    let mut para = Paragraph::new_empty();
    para.char_count_msb = true;
    para.char_shapes.clear();
    para.line_segs.clear();
    let mut last_end = top;
    let mut utf16 = 0;
    for (line_no, line) in lines.iter().enumerate() {
        let indices = refs(&line["spanIndices"])?;
        if indices.is_empty() {
            return Err("empty recognized line".into());
        }
        let baseline = number(&line["baseline"])?;
        let size = number(&line["fontSize"])?;
        let line_top = baseline - size * 0.95;
        let next_top = if let Some(next) = lines.get(line_no + 1) {
            number(&next["baseline"])? - number(&next["fontSize"])? * 0.95
        } else {
            line_top + size
        };
        let (text, runs) = text_with_spaces(page, &indices);
        // Preserve explicit PDF line grouping; paragraph boundaries remain heuristic.
        if line_no > 0 {
            para.text.push('\n');
            utf16 += 1;
        }
        let line_start = utf16;
        for (run_offset, i) in runs {
            let style = shape_ids[i].ok_or("span style was not generated")?;
            para.char_shapes.push(CharShapeRef {
                start_pos: utf16 + run_offset,
                char_shape_id: style,
            });
        }
        para.text.push_str(&text);
        utf16 += text.encode_utf16().count() as u32;
        para.line_segs.push(LineSeg {
            text_start: line_start,
            vertical_pos: hu(line_top - top)? as i32,
            line_height: hu(size)? as i32,
            text_height: hu(size)? as i32,
            baseline_distance: hu(size * 0.95)? as i32,
            line_spacing: hu((next_top - line_top - size).max(0.))? as i32,
            column_start: if centered {
                0
            } else {
                hu(number(&line["bbox"]["x"])? - cell_x)? as i32
            },
            segment_width: if centered {
                hu(cell_width)? as i32
            } else {
                hu(cell_width - (number(&line["bbox"]["x"])? - cell_x))? as i32
            },
            tag: LineSeg::TAG_SINGLE_SEGMENT_LINE,
        });
        last_end = line_top + size;
    }
    let (char_offsets, count) = offsets(&para.text);
    para.char_offsets = char_offsets;
    para.char_count = count + 1;
    para.has_para_text = true;
    let mut style = doc.doc_info.para_shapes[0].clone();
    style.raw_data = None;
    style.attr2 = (style.attr2 & !PARA_ATTR2_LINE_WRAP_MASK) | 1;
    style.alignment = if centered {
        Alignment::Center
    } else {
        Alignment::Left
    };
    style.margin_left = if centered {
        0
    } else {
        hu(2. * (number(&first["bbox"]["x"])? - cell_x))? as i32
    };
    style.margin_right = 0;
    style.indent = 0;
    style.spacing_before = 0;
    style.spacing_after = 0;
    style.line_spacing_type = LineSpacingType::Percent;
    style.line_spacing = if lines.len() > 1 {
        ((number(&lines[1]["baseline"])? - number(&first["baseline"])?)
            / number(&first["fontSize"])?
            * 100.)
            .round() as i32
    } else {
        100
    };
    para.para_shape_id = doc.doc_info.para_shapes.len() as u16;
    doc.doc_info.para_shapes.push(style);
    Ok(FlowItem {
        top,
        bottom: last_end,
        paragraph: para,
    })
}

fn is_grid_rule(shape: &ShapeObject, grid: &Value) -> Result<bool, String> {
    let (x1, y1, x2, y2) = match shape {
        ShapeObject::Line(line) => (
            (line.common.horizontal_offset as f32 + line.start.x as f32) / 100.,
            (line.common.vertical_offset as f32 + line.start.y as f32) / 100.,
            (line.common.horizontal_offset as f32 + line.end.x as f32) / 100.,
            (line.common.vertical_offset as f32 + line.end.y as f32) / 100.,
        ),
        ShapeObject::Rectangle(r)
            if r.drawing.text_box.is_none() && r.drawing.fill.solid.is_some() =>
        {
            let c = &r.common;
            let (x, y, w, h) = (
                c.horizontal_offset as f32 / 100.,
                c.vertical_offset as f32 / 100.,
                c.width as f32 / 100.,
                c.height as f32 / 100.,
            );
            if w < h {
                (x + w / 2., y, x + w / 2., y + h)
            } else {
                (x, y + h / 2., x + w, y + h / 2.)
            }
        }
        _ => return Ok(false),
    };
    // Only consume axes within the reconstructed grid, never arbitrary interior artwork.
    let b = &grid["bbox"];
    let inside = x1.min(x2) >= number(&b["x"])? - 0.5
        && x1.max(x2) <= number(&b["x"])? + number(&b["width"])? + 0.5
        && y1.min(y2) >= number(&b["y"])? - 0.5
        && y1.max(y2) <= number(&b["y"])? + number(&b["height"])? + 0.5;
    if !inside {
        return Ok(false);
    }
    let on_axis = |values: &Value, at: f32| -> Result<bool, String> {
        Ok(array(values)?
            .iter()
            .any(|v| number(v).is_ok_and(|n| (n - at).abs() <= 0.5)))
    };
    Ok(
        ((y1 - y2).abs() <= 0.5 && on_axis(&grid["yBoundaries"], y1)?)
            || ((x1 - x2).abs() <= 0.5 && on_axis(&grid["xBoundaries"], x1)?),
    )
}

pub(crate) fn lower(page: &ExtractedPage) -> Result<(Document, Value), String> {
    let (mut doc, mut report) = crate::lowering::lower(page, true)?;
    let controls = std::mem::take(&mut doc.sections[0].paragraphs[0].controls);
    let mut span_controls = vec![None; page.spans.len()];
    let mut shape_ids = vec![None; page.spans.len()];
    let mut image_controls = Vec::new();
    let mut span_cursor = 0;
    let nonempty: Vec<_> = page
        .spans
        .iter()
        .enumerate()
        .filter_map(|(i, s)| (!s.text.is_empty()).then_some(i))
        .collect();
    for (control_idx, c) in controls.iter().enumerate() {
        match c {
            Control::Picture(_) => image_controls.push(control_idx),
            Control::Shape(s) => {
                if let Some(box_) = s.drawing().and_then(|d| d.text_box.as_ref()) {
                    let source = *nonempty
                        .get(span_cursor)
                        .ok_or("span/control map mismatch")?;
                    span_controls[source] = Some(control_idx);
                    shape_ids[source] = Some(box_.paragraphs[0].char_shapes[0].char_shape_id);
                    span_cursor += 1;
                }
            }
            _ => {}
        }
    }
    let grids = array(&page.snapshot["tableDetection"]["tables"])?;
    let recognized = array(&page.snapshot["recognition"]["tables"])?;
    let mut removed = BTreeSet::new();
    let mut tables = Vec::new();
    let mut held = Vec::new();
    for recognized_table in recognized {
        let candidate = index(&recognized_table["candidate"])?;
        if recognized_table["status"] != "GeometryConfirmed"
            || array(&recognized_table["cells"])?.iter().any(|c| {
                c["recognition"]["paragraphs"]
                    .as_array()
                    .is_none_or(|ps| ps.iter().any(|p| p["status"] != "Confirmed"))
            })
        {
            held.push(json!({"candidate":candidate,"reason":"recognition_deferred"}));
            continue;
        }
        let grid = grids.get(candidate).ok_or("missing recognized grid")?;
        let style_start = doc.doc_info.para_shapes.len();
        let border_start = doc.doc_info.border_fills.len();
        let mut candidate_removed = BTreeSet::new();
        let built = (|| -> Result<Control, String> {
            let mut stroke_styles = BTreeSet::new();
            for c in &controls {
                if let Control::Shape(s) = c {
                    if is_grid_rule(s, grid)? {
                        match s.as_ref() {
                            ShapeObject::Line(l) => {
                                stroke_styles.insert((
                                    l.drawing.border_line.color,
                                    l.drawing.border_line.width as u32,
                                ));
                            }
                            ShapeObject::Rectangle(r) => {
                                stroke_styles.insert((
                                    r.drawing
                                        .fill
                                        .solid
                                        .as_ref()
                                        .ok_or("missing rule fill")?
                                        .background_color,
                                    r.common.width.min(r.common.height),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
            }
            if stroke_styles.len() != 1 {
                return Err("mixed or missing rule styles require further lowering".into());
            }
            let &(color, width) = stroke_styles.first().ok_or("missing rule style")?;
            let mut border = doc
                .doc_info
                .border_fills
                .get(1)
                .ok_or("missing scaffold solid border")?
                .clone();
            border.raw_data = None;
            for side in &mut border.borders {
                side.color = color;
                side.width = border_width_index(width as f64 / 100. * 25.4 / 72.);
            }
            doc.doc_info.border_fills.push(border);
            let border_id =
                u16::try_from(doc.doc_info.border_fills.len()).map_err(|e| e.to_string())?;
            let b = &grid["bbox"];
            let mut table = Table {
                row_count: u16::try_from(index(&grid["rows"])?).map_err(|e| e.to_string())?,
                col_count: u16::try_from(index(&grid["columns"])?).map_err(|e| e.to_string())?,
                border_fill_id: border_id,
                page_break: TablePageBreak::None,
                common: CommonObjAttr {
                    width: hu(number(&b["width"])?)?,
                    height: hu(number(&b["height"])?)?,
                    horizontal_offset: hu(number(&b["x"])?)?,
                    vertical_offset: hu(number(&b["y"])?)?,
                    instance_id: u32::try_from(controls.len() + 2 + candidate * 2)
                        .map_err(|e| e.to_string())?,
                    text_wrap: TextWrap::InFrontOfText,
                    allow_overlap: true,
                    ..Default::default()
                },
                ..Default::default()
            };
            table.row_sizes = vec![table.col_count as i16; table.row_count as usize];
            for recognized_cell in array(&recognized_table["cells"])? {
                let cell_idx = index(&recognized_cell["cell"])?;
                let source = array(&grid["cells"])?
                    .get(cell_idx)
                    .ok_or("missing cell geometry")?;
                let cb = &source["bbox"];
                let mut cell = Cell::new_empty(
                    index(&source["column"])? as u16,
                    index(&source["row"])? as u16,
                    hu(number(&cb["width"])?)?,
                    hu(number(&cb["height"])?)?,
                    border_id,
                );
                cell.col_span = index(&source["colspan"])? as u16;
                cell.row_span = index(&source["rowspan"])? as u16;
                cell.vertical_align = VerticalAlign::Top;
                cell.apply_inner_margin = true;
                cell.paragraphs[0].char_shapes = vec![CharShapeRef {
                    start_pos: 0,
                    char_shape_id: 0,
                }];
                let mut flow = Vec::new();
                for proposal in array(&recognized_cell["recognition"]["paragraphs"])? {
                    flow.push(paragraph(page, &mut doc, proposal, cb, &shape_ids)?);
                }
                for image_idx in refs(&source["imageIndices"])? {
                    let control_idx = *image_controls
                        .get(image_idx)
                        .ok_or("image/control map mismatch")?;
                    let Control::Picture(original) = &controls[control_idx] else {
                        unreachable!()
                    };
                    let mut picture = original.clone();
                    let top = picture.common.vertical_offset as f32 / 100.;
                    picture.common.treat_as_char = true;
                    picture.common.text_wrap = TextWrap::TopAndBottom;
                    picture.common.allow_overlap = false;
                    picture.common.vert_rel_to = VertRelTo::Para;
                    picture.common.horz_rel_to = HorzRelTo::Para;
                    picture.common.horizontal_offset = 0;
                    picture.common.vertical_offset = 0;
                    let mut para = Paragraph::new_empty();
                    para.char_count_msb = true;
                    para.char_count = 9;
                    para.control_mask = 0x800;
                    para.char_shapes = vec![CharShapeRef {
                        start_pos: 0,
                        char_shape_id: 0,
                    }];
                    let mut ps = doc.doc_info.para_shapes[0].clone();
                    ps.raw_data = None;
                    ps.margin_left = 2
                        * (original.common.horizontal_offset as i32
                            - hu(number(&cb["x"])?)? as i32);
                    ps.line_spacing = 100;
                    ps.spacing_before = 0;
                    ps.spacing_after = 0;
                    para.para_shape_id = doc.doc_info.para_shapes.len() as u16;
                    doc.doc_info.para_shapes.push(ps);
                    para.line_segs = vec![LineSeg {
                        line_height: picture.common.height as i32,
                        text_height: picture.common.height as i32,
                        baseline_distance: picture.common.height as i32,
                        segment_width: cell.width as i32,
                        tag: LineSeg::TAG_SINGLE_SEGMENT_LINE,
                        ..Default::default()
                    }];
                    let bottom = top + picture.common.height as f32 / 100.;
                    para.controls.push(Control::Picture(picture));
                    flow.push(FlowItem {
                        top,
                        bottom,
                        paragraph: para,
                    });
                    candidate_removed.insert(control_idx);
                }
                flow.sort_by(|a, b| a.top.total_cmp(&b.top));
                let mut cursor = number(&cb["y"])?;
                let mut previous_style: Option<u16> = None;
                for (flow_index, item) in flow.iter_mut().enumerate() {
                    if item.top < cursor - 0.5 {
                        return Err("overlapping cell flow requires additional lowering".into());
                    }
                    let gap = (item.top - cursor).max(0.);
                    let picture = item
                        .paragraph
                        .controls
                        .first()
                        .is_some_and(|c| matches!(c, Control::Picture(_)));
                    if picture {
                        // Place the inter-item gap before the inline image's flow origin.
                        if let Some(style) = previous_style {
                            doc.doc_info.para_shapes[style as usize].spacing_after =
                                hu(2. * gap)? as i32;
                        } else {
                            cell.padding.top = i16::try_from(hu(gap)?)
                                .map_err(|_| "cell padding exceeds format range")?;
                        }
                    } else {
                        doc.doc_info.para_shapes[item.paragraph.para_shape_id as usize]
                            .spacing_before = hu(2. * gap)? as i32;
                    }
                    if flow_index == 0 && item.paragraph.controls.is_empty() {
                        // Stored cell starts own their lead in the frame, not flow_before.
                        let lead = hu((item.top - cursor).max(0.))? as i32;
                        for segment in &mut item.paragraph.line_segs {
                            segment.vertical_pos += lead;
                        }
                    }
                    cursor = item.bottom;
                    previous_style = Some(item.paragraph.para_shape_id);
                }
                if !flow.is_empty() {
                    cell.paragraphs = flow.into_iter().map(|i| i.paragraph).collect();
                }
                for i in refs(&source["spanIndices"])? {
                    if let Some(c) = span_controls[i] {
                        candidate_removed.insert(c);
                    }
                }
                table.cells.push(cell);
            }
            table.rebuild_grid();
            table.row_sizes = (0..table.row_count)
                .map(|row| table.cells.iter().filter(|c| c.row == row).count() as i16)
                .collect();
            for (i, c) in controls.iter().enumerate() {
                if let Control::Shape(s) = c {
                    if is_grid_rule(s, grid)? {
                        candidate_removed.insert(i);
                    }
                }
            }
            Ok(Control::Table(Box::new(table)))
        })();
        match built {
            Ok(table) => {
                removed.extend(candidate_removed);
                tables.push(table);
            }
            Err(reason) => {
                doc.doc_info.para_shapes.truncate(style_start);
                doc.doc_info.border_fills.truncate(border_start);
                held.push(json!({"candidate":candidate,"reason":reason}));
            }
        }
    }
    let table_count = tables.len();
    let mut retained: Vec<_> = controls
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| (!removed.contains(&i)).then_some(c))
        .collect();
    retained.extend(tables);
    let owner = &mut doc.sections[0].paragraphs[0];
    owner.controls = retained;
    owner.char_count = 1 + owner.controls.len() as u32 * 8;
    report["mode"] = json!("structured-tables-with-positional-fallback");
    report["semanticTables"] = json!(table_count);
    report["ruledTableCandidates"] = json!(grids.len());
    report["tableSemantics"] = json!("inferred-from-geometry-not-authored-metadata");
    report["textBoxes"] = json!(owner
        .controls
        .iter()
        .filter(|c| matches!(c,
        Control::Shape(s) if s.drawing().is_some_and(|d| d.text_box.is_some())))
        .count());
    report["straightRules"] = json!(owner
        .controls
        .iter()
        .filter(|c| matches!(c,
        Control::Shape(s) if matches!(s.as_ref(), ShapeObject::Line(_))))
        .count());
    report["paintedRuleRectangles"]=json!(owner.controls.iter().filter(|c|matches!(c,
        Control::Shape(s) if matches!(s.as_ref(),ShapeObject::Rectangle(r) if r.drawing.text_box.is_none() && r.drawing.fill.solid.is_some()))).count());
    report["deferredTables"] = json!(held);
    report["hancomCompatibility"] = json!("unverified-awaiting-editor-test");
    report["warnings"] = json!([
        "PDF table/paragraph semantics are inferred, not recovered authored metadata",
        "PDF fonts, exact glyph advances and paragraph formatting are not fully reconstructed",
        "Only selected source page is converted; Hancom editor interoperability is unverified"
    ]);
    Ok((doc, report))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pdf_oxide::elements::{PathContent, PathOperation};

    fn grid() -> Vec<PathContent> {
        [
            (10., 10., 70., 10.),
            (10., 40., 70., 40.),
            (10., 70., 70., 70.),
            (10., 10., 10., 70.),
            (40., 10., 40., 70.),
            (70., 10., 70., 70.),
        ]
        .into_iter()
        .map(|(x, y, xx, yy)| {
            PathContent::from_operations(vec![
                PathOperation::MoveTo(x, y),
                PathOperation::LineTo(xx, yy),
            ])
        })
        .collect()
    }
    fn page(paths: Vec<PathContent>) -> ExtractedPage {
        let media_box = (0., 0., 200., 200.);
        let detection =
            crate::table_detection::detect_ruled_tables(&[], &paths, &[], media_box).unwrap();
        let recognition = crate::recognition::recognize_page(
            &[],
            &[],
            &detection,
            media_box,
            crate::recognition::Policy::default(),
        )
        .unwrap();
        ExtractedPage {
            media_box,
            spans: vec![],
            images: vec![],
            paths,
            snapshot: json!({"tableDetection":detection,"recognition":recognition,"tables":[]}),
        }
    }
    #[test]
    fn empty_confirmed_grid_becomes_cells_and_merged_ownership_is_preserved() {
        let mut paths = grid();
        paths[4] = PathContent::from_operations(vec![
            PathOperation::MoveTo(40., 10.),
            PathOperation::LineTo(40., 40.),
        ]);
        let (doc, report) = lower(&page(paths)).unwrap();
        assert_eq!(report["semanticTables"], 1);
        let Control::Table(t) = &doc.sections[0].paragraphs[0].controls[0] else {
            panic!("actual table required")
        };
        assert_eq!((t.row_count, t.col_count, t.cells.len()), (2, 2, 3));
        assert!(t.cells.iter().any(|c| c.col_span == 2));
        assert_eq!(t.cell_grid[0], t.cell_grid[1]);
        assert!(t.cells.iter().all(|c| c.paragraphs.len() == 1));
    }
    #[test]
    fn deferred_candidate_keeps_every_positional_control() {
        let mut p = page(grid());
        p.snapshot["recognition"]["tables"][0]["status"] = json!("Deferred");
        let (baseline, _) = crate::lowering::lower(&p, true).unwrap();
        let (doc, report) = lower(&p).unwrap();
        assert_eq!(report["semanticTables"], 0);
        assert_eq!(report["deferredTables"].as_array().unwrap().len(), 1);
        assert_eq!(
            serde_json::to_value(&doc.sections[0].paragraphs).unwrap(),
            serde_json::to_value(&baseline.sections[0].paragraphs).unwrap()
        );
    }

    #[test]
    fn rowspan_keeps_all_logical_slots() {
        let mut paths = grid();
        paths[1] = PathContent::from_operations(vec![
            PathOperation::MoveTo(40., 40.),
            PathOperation::LineTo(70., 40.),
        ]);
        let (doc, report) = lower(&page(paths)).unwrap();
        assert_eq!(report["semanticTables"], 1);
        let Control::Table(t) = &doc.sections[0].paragraphs[0].controls[0] else {
            panic!("actual table required")
        };
        assert_eq!(t.cells.len(), 3);
        assert!(t.cells.iter().any(|c| c.row_span == 2));
        assert_eq!(t.cell_grid[0], t.cell_grid[2]);
    }

    #[test]
    fn mixed_rule_styles_are_deferred_without_color_loss() {
        let mut paths = grid();
        paths[0].stroke_color = Some(pdf_oxide::layout::Color {
            r: 1.,
            g: 0.,
            b: 0.,
        });
        let p = page(paths);
        let (baseline, _) = crate::lowering::lower(&p, true).unwrap();
        let (doc, report) = lower(&p).unwrap();
        assert_eq!(report["semanticTables"], 0);
        assert_eq!(report["deferredTables"].as_array().unwrap().len(), 1);
        assert_eq!(
            serde_json::to_value(&doc.sections[0].paragraphs).unwrap(),
            serde_json::to_value(&baseline.sections[0].paragraphs).unwrap()
        );
    }
    #[test]
    fn failed_candidate_does_not_drop_rules_or_leave_styles() {
        let mut p = page(grid());
        p.snapshot["tableDetection"]["tables"][0]["bbox"]["width"] = json!(-1);
        let (baseline, _) = crate::lowering::lower(&p, true).unwrap();
        let (doc, report) = lower(&p).unwrap();
        assert_eq!(report["semanticTables"], 0);
        assert_eq!(
            serde_json::to_value(&doc.doc_info.para_shapes).unwrap(),
            serde_json::to_value(&baseline.doc_info.para_shapes).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&doc.sections[0].paragraphs).unwrap(),
            serde_json::to_value(&baseline.sections[0].paragraphs).unwrap()
        );
    }
}
