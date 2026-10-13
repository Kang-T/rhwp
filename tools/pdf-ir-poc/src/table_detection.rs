//! Detection only: vector-rule topology, independent of text wrapping and IR lowering.
use pdf_oxide::{
    elements::{PathContent, PathOperation},
    extractors::PdfImage,
    geometry::Rect,
    layout::TextSpan,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const EPS: f32 = 0.5;
const MAX_EDGES: usize = 512;
const MAX_TILES: usize = 4096;

#[derive(Clone, Copy)]
struct Edge {
    horizontal: bool,
    at: f32,
    from: f32,
    to: f32,
}

struct Sets(Vec<usize>);
impl Sets {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }
    fn root(&mut self, mut i: usize) -> usize {
        while self.0[i] != i {
            self.0[i] = self.0[self.0[i]];
            i = self.0[i];
        }
        i
    }
    fn join(&mut self, a: usize, b: usize) {
        let a = self.root(a);
        let b = self.root(b);
        self.0[b] = a;
    }
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = BTreeMap::<usize, Vec<usize>>::new();
        for i in 0..self.0.len() {
            let root = self.root(i);
            groups.entry(root).or_default().push(i);
        }
        groups.into_values().collect()
    }
}

fn edges(paths: &[PathContent], left: f32, top: f32) -> Result<Vec<Edge>, String> {
    let mut edges = Vec::new();
    let mut invalid = false;
    let mut overflow = false;
    let mut add = |x1: f32, y1: f32, x2: f32, y2: f32| {
        if ![x1, y1, x2, y2].iter().all(|v| v.is_finite()) {
            invalid = true;
            return;
        }
        if edges.len() >= MAX_EDGES {
            overflow = true;
            return;
        }
        let (x1, x2, y1, y2) = (x1 - left, x2 - left, top - y1, top - y2);
        if (y1 - y2).abs() <= EPS && (x1 - x2).abs() > EPS {
            edges.push(Edge {
                horizontal: true,
                at: (y1 + y2) / 2.0,
                from: x1.min(x2),
                to: x1.max(x2),
            });
        } else if (x1 - x2).abs() <= EPS && (y1 - y2).abs() > EPS {
            edges.push(Edge {
                horizontal: false,
                at: (x1 + x2) / 2.0,
                from: y1.min(y2),
                to: y1.max(y2),
            });
        }
    };
    for path in paths {
        if path.stroke_color.is_none() || path.matrix.is_some() {
            continue;
        }
        let mut cursor = None;
        let mut start = None;
        for op in &path.operations {
            match *op {
                PathOperation::MoveTo(x, y) => {
                    cursor = Some((x, y));
                    start = cursor;
                }
                PathOperation::LineTo(x, y) => {
                    if let Some((px, py)) = cursor {
                        add(px, py, x, y);
                    }
                    cursor = Some((x, y));
                }
                PathOperation::Rectangle(x, y, w, h) => {
                    add(x, y, x + w, y);
                    add(x + w, y, x + w, y + h);
                    add(x + w, y + h, x, y + h);
                    add(x, y + h, x, y);
                    cursor = Some((x, y));
                    start = cursor;
                }
                PathOperation::ClosePath => {
                    if let (Some((x, y)), Some((sx, sy))) = (cursor, start) {
                        add(x, y, sx, sy);
                    }
                    cursor = start;
                }
                PathOperation::CurveTo(..) => {
                    cursor = None;
                    start = None;
                }
            }
        }
    }
    if invalid {
        return Err("non-finite rule geometry".into());
    }
    if overflow {
        return Err("ruled-table edge budget exceeded".into());
    }
    Ok(edges)
}

fn axes(mut values: Vec<f32>) -> Vec<f32> {
    values.sort_by(f32::total_cmp);
    let mut groups: Vec<Vec<f32>> = Vec::new();
    for v in values {
        if let Some(group) = groups.last_mut().filter(|g| v - g[0] <= EPS) {
            group.push(v);
        } else {
            groups.push(vec![v]);
        }
    }
    groups
        .iter()
        .map(|g| g.iter().sum::<f32>() / g.len() as f32)
        .collect()
}

fn coverage(edges: &[Edge], horizontal: bool, at: f32, from: f32, to: f32) -> f32 {
    let mut intervals: Vec<_> = edges
        .iter()
        .filter(|e| e.horizontal == horizontal && (e.at - at).abs() <= EPS)
        .map(|e| (e.from.max(from), e.to.min(to)))
        .filter(|(a, b)| b > a)
        .collect();
    intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut total = 0.0;
    let mut end = from;
    for (a, b) in intervals {
        if a > end + EPS {
            total += (b - a).max(0.0);
        } else {
            total += (b - end).max(0.0);
        }
        end = end.max(b);
    }
    total
}

fn complete(edges: &[Edge], horizontal: bool, at: f32, from: f32, to: f32) -> bool {
    coverage(edges, horizontal, at, from, to) >= to - from - EPS
}

fn inside(inner: Rect, outer: Rect) -> bool {
    inner.x >= outer.x - EPS
        && inner.y >= outer.y - EPS
        && inner.x + inner.width <= outer.x + outer.width + EPS
        && inner.y + inner.height <= outer.y + outer.height + EPS
}

fn overlaps(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height
}

struct Cell {
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    bbox: Rect,
    spans: Vec<usize>,
    images: Vec<usize>,
}
struct Grid {
    bbox: Rect,
    xs: Vec<f32>,
    ys: Vec<f32>,
    cells: Vec<Cell>,
}

fn grid(edges: &[Edge]) -> Result<Grid, String> {
    let xs = axes(
        edges
            .iter()
            .filter(|e| !e.horizontal)
            .map(|e| e.at)
            .collect(),
    );
    let ys = axes(
        edges
            .iter()
            .filter(|e| e.horizontal)
            .map(|e| e.at)
            .collect(),
    );
    if xs.len() < 3 || ys.len() < 3 {
        return Err("not a two-dimensional multi-cell grid".into());
    }
    let (cols, rows) = (xs.len() - 1, ys.len() - 1);
    if cols * rows > MAX_TILES {
        return Err("ruled-table tile budget exceeded".into());
    }
    if !complete(edges, true, ys[0], xs[0], xs[cols])
        || !complete(edges, true, ys[rows], xs[0], xs[cols])
        || !complete(edges, false, xs[0], ys[0], ys[rows])
        || !complete(edges, false, xs[cols], ys[0], ys[rows])
    {
        return Err("open outer boundary".into());
    }
    let mut sets = Sets::new(rows * cols);
    // A missing internal rule joins neighboring tiles; a partial rule is ambiguous, not an invented merge.
    for row in 0..rows {
        for col in 0..cols {
            if col + 1 < cols {
                let length = ys[row + 1] - ys[row];
                let covered = coverage(edges, false, xs[col + 1], ys[row], ys[row + 1]);
                if covered <= EPS {
                    sets.join(row * cols + col, row * cols + col + 1);
                } else if covered < length - EPS {
                    return Err("partial internal vertical boundary".into());
                }
            }
            if row + 1 < rows {
                let length = xs[col + 1] - xs[col];
                let covered = coverage(edges, true, ys[row + 1], xs[col], xs[col + 1]);
                if covered <= EPS {
                    sets.join(row * cols + col, (row + 1) * cols + col);
                } else if covered < length - EPS {
                    return Err("partial internal horizontal boundary".into());
                }
            }
        }
    }
    let mut cells = Vec::new();
    for group in sets.groups() {
        let row = group.iter().map(|i| i / cols).min().unwrap();
        let col = group.iter().map(|i| i % cols).min().unwrap();
        let end_row = group.iter().map(|i| i / cols).max().unwrap() + 1;
        let end_col = group.iter().map(|i| i % cols).max().unwrap() + 1;
        if group.len() != (end_row - row) * (end_col - col) {
            return Err("non-rectangular cell topology".into());
        }
        cells.push(Cell {
            row,
            col,
            rows: end_row - row,
            cols: end_col - col,
            bbox: Rect::new(
                xs[col],
                ys[row],
                xs[end_col] - xs[col],
                ys[end_row] - ys[row],
            ),
            spans: Vec::new(),
            images: Vec::new(),
        });
    }
    if cells.len() < 2 {
        return Err("no distinct cells".into());
    }
    cells.sort_by_key(|c| (c.row, c.col));
    Ok(Grid {
        bbox: Rect::new(xs[0], ys[0], xs[cols] - xs[0], ys[rows] - ys[0]),
        xs,
        ys,
        cells,
    })
}

/// Returns geometric table candidates and original content indices, not rhwp Table IR.
pub fn detect_ruled_tables(
    spans: &[TextSpan],
    paths: &[PathContent],
    images: &[PdfImage],
    media_box: (f32, f32, f32, f32),
) -> Result<Value, String> {
    let (left, bottom, right, top) = media_box;
    if ![left, bottom, right, top].iter().all(|v| v.is_finite()) || right <= left || top <= bottom {
        return Err("invalid page geometry".into());
    }
    let (left, _, _, top) = media_box;
    let edges = edges(paths, left, top)?;
    let mut sets = Sets::new(edges.len());
    for a in 0..edges.len() {
        for b in a + 1..edges.len() {
            let (a_edge, b_edge) = (edges[a], edges[b]);
            let touches = if a_edge.horizontal == b_edge.horizontal {
                (a_edge.at - b_edge.at).abs() <= EPS
                    && a_edge.from <= b_edge.to + EPS
                    && b_edge.from <= a_edge.to + EPS
            } else {
                let (h, v) = if a_edge.horizontal {
                    (a_edge, b_edge)
                } else {
                    (b_edge, a_edge)
                };
                v.at >= h.from - EPS
                    && v.at <= h.to + EPS
                    && h.at >= v.from - EPS
                    && h.at <= v.to + EPS
            };
            if touches {
                sets.join(a, b);
            }
        }
    }
    let mut grids = Vec::new();
    let mut rejected = Vec::new();
    for group in sets.groups() {
        let component: Vec<_> = group.iter().map(|i| edges[*i]).collect();
        match grid(&component) {
            Ok(g) => grids.push(g),
            Err(reason) => rejected.push(json!({"edgeCount":component.len(),"reason":reason})),
        }
    }
    grids.sort_by(|a, b| {
        a.bbox
            .y
            .total_cmp(&b.bbox.y)
            .then(a.bbox.x.total_cmp(&b.bbox.x))
    });
    let mut unassigned = Vec::new();
    let mut ambiguous = Vec::new();
    let mut assign = |kind: &str, index: usize, bbox: Rect| {
        let mut owners = Vec::new();
        for (gi, g) in grids.iter().enumerate() {
            if !inside(bbox, g.bbox) {
                continue;
            }
            for (ci, c) in g.cells.iter().enumerate() {
                if inside(bbox, c.bbox) {
                    owners.push((gi, ci));
                }
            }
        }
        if owners.len() == 1 {
            let (gi, ci) = owners[0];
            let cell = &mut grids[gi].cells[ci];
            if kind == "span" {
                cell.spans.push(index);
            } else {
                cell.images.push(index);
            }
        } else if grids.iter().any(|g| overlaps(bbox, g.bbox)) {
            ambiguous.push(json!({"kind":kind,"index":index,"ownerCount":owners.len()}));
        } else {
            unassigned.push(json!({"kind":kind,"index":index}));
        }
    };
    for (i, s) in spans.iter().enumerate() {
        if !s.text.is_empty() {
            if ![s.bbox.x, s.bbox.y, s.bbox.width, s.bbox.height]
                .iter()
                .all(|v| v.is_finite())
                || s.bbox.width < 0.0
                || s.bbox.height < 0.0
                || s.rotation_degrees != 0.0
                || s.wmode != 0
                || s.mirrored
            {
                return Err("unsupported text geometry in table detection".into());
            }
            assign(
                "span",
                i,
                Rect::new(
                    s.bbox.x - left,
                    top - s.bbox.y - s.bbox.height,
                    s.bbox.width,
                    s.bbox.height,
                ),
            );
        }
    }
    for (i, image) in images.iter().enumerate() {
        if let Some(b) = image.bbox() {
            if ![b.x, b.y, b.width, b.height].iter().all(|v| v.is_finite())
                || b.width <= 0.0
                || b.height <= 0.0
            {
                return Err("invalid image geometry".into());
            }
            assign(
                "image",
                i,
                Rect::new(b.x - left, top - b.y - b.height, b.width, b.height),
            );
        } else {
            return Err("image placement unavailable for table ownership".into());
        }
    }
    Ok(json!({
        "algorithm":"ruling-topology-v1","coordinateSystem":"points, top-left of MediaBox",
        "scope":"stroked axis-aligned ruled grids; geometric candidates, not semantic proof",
        "tolerancePt":EPS,"edgeCount":edges.len(),
        "tables":grids.iter().map(|g|json!({
            "bbox":g.bbox,"rows":g.ys.len()-1,"columns":g.xs.len()-1,
            "xBoundaries":g.xs,"yBoundaries":g.ys,
            "cells":g.cells.iter().map(|c|json!({"row":c.row,"column":c.col,
                "rowspan":c.rows,"colspan":c.cols,"bbox":c.bbox,
                "spanIndices":c.spans,"imageIndices":c.images
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "contentOrder":"original extraction index; semantic reading order deferred",
        "rejectedComponents":rejected,"unassignedContent":unassigned,"ambiguousContent":ambiguous
    }))
}
