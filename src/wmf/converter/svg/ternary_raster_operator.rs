use crate::wmf::converter::{
    svg::{device_context::BlitDestRect, node::Node, util::url_string, Fill},
    *,
};

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum TernaryRasterOperationError {
    #[snafu(display("no brush specified: {cause}"))]
    NoBrush { cause: String },
    #[snafu(display("no source bitmap specified: {cause}"))]
    NoSource { cause: String },
}

#[derive(PartialEq)]
struct BrushOnlyRopKey {
    rect: [i32; 4],
    fill: String,
    clip_id: Option<String>,
    clip_rect: Option<[i32; 4]>,
}

#[derive(Default)]
pub struct BrushOnlyRopSequence {
    state: Option<BrushOnlyRopSequenceState>,
    suppressed_elements: Vec<usize>,
    vector_masks: Vec<(usize, Node, String, [i32; 4])>,
}

enum BrushOnlyRopSequenceState {
    AwaitDpa {
        key: BrushOnlyRopKey,
        expected_element_count: usize,
    },
    AwaitFinalPatInvert {
        key: BrushOnlyRopKey,
        expected_element_count: usize,
        mask: Option<Node>,
    },
    AwaitFinalVectorInvert {
        key: BrushOnlyRopKey,
        expected_element_count: usize,
        mask: Node,
    },
}

impl BrushOnlyRopSequence {
    /// 삼중 연산이 확정될 때까지 가운데 칠을 보존한다. 흑백 마스크는
    /// 흰 부분의 기존 배경과 검은 부분의 브러시 색을 구분하는 소유 정보다.
    /// 최종 XOR만 즉시 생략하고 확정된 마스크는 SVG 마감 시 칠에 적용한다.
    fn observe(
        &mut self,
        operation: TernaryRasterOperation,
        key: BrushOnlyRopKey,
        element_count: usize,
        has_monochrome_pattern: bool,
        allow_mask_edge_delta: bool,
    ) -> bool {
        let state = std::mem::take(&mut self.state);
        match (state, operation) {
            (
                Some(BrushOnlyRopSequenceState::AwaitFinalVectorInvert {
                    key: previous_key,
                    expected_element_count,
                    mask,
                }),
                TernaryRasterOperation::PATINVERT,
            ) if key == previous_key && element_count == expected_element_count => {
                // (D ⊕ P) ∧ M ⊕ P는 흰 마스크에서 D, 검은 마스크에서 P다.
                // 두 바깥 XOR 사각형 대신 같은 윤곽의 역흑백 마스크만 표시한다.
                self.suppressed_elements.push(element_count - 2);
                self.vector_masks.push((
                    element_count - 1,
                    mask,
                    previous_key.fill,
                    previous_key.rect,
                ));
                true
            }
            (
                Some(BrushOnlyRopSequenceState::AwaitDpa {
                    key: previous_key,
                    expected_element_count,
                }),
                TernaryRasterOperation::DPA,
            ) if element_count == expected_element_count
                && key.clip_id == previous_key.clip_id
                && key.clip_rect == previous_key.clip_rect =>
            {
                // 같은 clip 안에서 확인한 흑백 패턴과 기존 경계 계약을 유지한다.
                // 마스크를 지우면 반톤도 전면 단색이 되므로 실제 가운데 칠을
                // 역흑백 마스크로 전달하고 그 패턴 정의도 보존한다.
                let mask = (has_monochrome_pattern
                    && Self::mask_rect_matches(key.rect, previous_key.rect, allow_mask_edge_delta))
                .then(|| {
                    let [x, y, width, height] = key.rect;
                    Node::new("rect")
                        .set("x", x)
                        .set("y", y)
                        .set("width", width)
                        .set("height", height)
                        .set("fill", key.fill)
                });
                self.state = Some(BrushOnlyRopSequenceState::AwaitFinalPatInvert {
                    key: previous_key,
                    expected_element_count: element_count + 1,
                    mask,
                });
                false
            }
            (
                Some(BrushOnlyRopSequenceState::AwaitFinalPatInvert {
                    key: previous_key,
                    expected_element_count,
                    mask,
                }),
                TernaryRasterOperation::PATINVERT,
            ) if key == previous_key && element_count == expected_element_count => {
                if let Some(mask) = mask {
                    self.suppressed_elements.push(element_count - 2);
                    self.vector_masks.push((
                        element_count - 1,
                        mask,
                        previous_key.fill,
                        previous_key.rect,
                    ));
                }
                true
            }
            (_, TernaryRasterOperation::PATINVERT) => {
                self.state = Some(BrushOnlyRopSequenceState::AwaitDpa {
                    key,
                    expected_element_count: element_count + 1,
                });
                false
            }
            _ => false,
        }
    }

    pub fn observe_vector_mask(
        &mut self,
        mask: &Node,
        brush: &Brush,
        draw_mode: Option<BinaryRasterOperation>,
        bounds: [i32; 4],
        clip: (Option<&str>, Option<&Rect>),
        element_count: usize,
    ) {
        let Some(BrushOnlyRopSequenceState::AwaitDpa {
            key,
            expected_element_count,
        }) = self.state.take()
        else {
            return;
        };
        let (clip_id, clip_rect) = clip;
        let clip_rect = clip_rect.map(|r| {
            [
                i32::from(r.left),
                i32::from(r.top),
                i32::from(r.right),
                i32::from(r.bottom),
            ]
        });
        let [x, y, width, height] = key.rect;
        if expected_element_count != element_count
            || draw_mode != Some(BinaryRasterOperation::R2_MASKPEN)
            || !Self::brush_is_monochrome_mask(Some(brush))
            || mask.attr("stroke") != Some("none")
            || !key.fill.starts_with('#')
            || key.clip_id.as_deref() != clip_id
            || key.clip_rect != clip_rect
            || width <= 0
            || height <= 0
            || bounds[0] < x
            || bounds[1] < y
            || i64::from(bounds[2]) > i64::from(x) + i64::from(width)
            || i64::from(bounds[3]) > i64::from(y) + i64::from(height)
        {
            return;
        }
        self.state = Some(BrushOnlyRopSequenceState::AwaitFinalVectorInvert {
            key,
            expected_element_count: element_count + 1,
            mask: mask.clone(),
        });
    }

    fn mask_rect_matches(mask: [i32; 4], paint: [i32; 4], allow_edge_delta: bool) -> bool {
        if mask == paint {
            return true;
        }
        // The issue6469 fixture has 8x8 halftone DPA rectangles whose
        // device-space edges differ by one unit from both outer XORs.
        // Do not extend this approximation to arbitrary masks or tiny draws.
        if !allow_edge_delta
            || [mask[2], mask[3], paint[2], paint[3]]
                .iter()
                .any(|&n| n < 8)
        {
            return false;
        }
        let edges = |[x, y, width, height]: [i32; 4]| {
            [
                i64::from(x),
                i64::from(y),
                i64::from(x) + i64::from(width),
                i64::from(y) + i64::from(height),
            ]
        };
        edges(mask)
            .into_iter()
            .zip(edges(paint))
            .all(|(a, b)| a.abs_diff(b) <= 1)
    }

    fn brush_is_8x8_halftone(brush: Option<&Brush>) -> bool {
        let Some(Brush::DIBPatternPT { brush_hatch, .. }) = brush else {
            return false;
        };
        if !matches!(
            &brush_hatch.dib_header_info,
            BitmapInfoHeader::Info(header)
                if header.planes == 1 && matches!(header.compression, Compression::BI_RGB)
        ) || brush_hatch.dib_header_info.width() != 8
            || brush_hatch.dib_header_info.height() != 8
        {
            return false;
        }
        let data = &brush_hatch.bitmap_buffer.a_data;
        if data.len() != 32 || !matches!(data[0], 0x55 | 0xAA) {
            return false;
        }
        data.chunks_exact(4)
            .enumerate()
            .all(|(row, bytes)| bytes[0] == if row % 2 == 0 { data[0] } else { !data[0] })
    }

    pub fn finish(
        self,
        mut definitions: Vec<Node>,
        mut elements: Vec<Node>,
    ) -> (Vec<Node>, Vec<Node>) {
        for (index, mask, color, [x, y, width, height]) in self.vector_masks {
            let filter_id = format!("rop_vector_inverse{}", definitions.len());
            let mask_id = format!("rop_vector_mask{}", definitions.len());
            let transfer = Node::new("feComponentTransfer")
                .add(
                    Node::new("feFuncR")
                        .set("type", "table")
                        .set("tableValues", "1 0"),
                )
                .add(
                    Node::new("feFuncG")
                        .set("type", "table")
                        .set("tableValues", "1 0"),
                )
                .add(
                    Node::new("feFuncB")
                        .set("type", "table")
                        .set("tableValues", "1 0"),
                );
            definitions.push(
                Node::new("filter")
                    .set("id", &filter_id)
                    .set("color-interpolation-filters", "sRGB")
                    .add(transfer),
            );
            definitions.push(
                Node::new("mask")
                    .set("id", &mask_id)
                    .set("maskUnits", "userSpaceOnUse")
                    .set("mask-type", "luminance")
                    .set("x", x)
                    .set("y", y)
                    .set("width", width)
                    .set("height", height)
                    .add(mask.set("filter", url_string(format!("#{filter_id}").as_str()))),
            );
            elements[index] = elements[index]
                .clone()
                .set("fill", color)
                .set("mask", url_string(format!("#{mask_id}").as_str()));
        }
        // 수집 중 요소를 지우면 이후 소유 인덱스가 바뀌므로 마감 시에만 제거한다.
        // 역마스크가 참조하는 패턴 정의는 같은 SVG 안에 남긴다.
        let elements = elements
            .into_iter()
            .enumerate()
            .filter_map(|(index, node)| {
                self.suppressed_elements
                    .binary_search(&index)
                    .is_err()
                    .then_some(node)
            })
            .collect();
        (definitions, elements)
    }

    /// One bit is a palette index, not proof of a black/white mask.
    /// Palette-indexed brushes cannot be classified without resolving the DC.
    fn brush_is_monochrome_mask(brush: Option<&Brush>) -> bool {
        let Some(Brush::DIBPatternPT {
            color_usage: ColorUsage::DIB_RGB_COLORS,
            brush_hatch,
        }) = brush
        else {
            return false;
        };
        if !matches!(
            brush_hatch.dib_header_info.bit_count(),
            BitCount::BI_BITCOUNT_1
        ) {
            return false;
        }
        let colors = match &brush_hatch.colors {
            Colors::RGBQuad(colors) if colors.len() == 2 => [
                [colors[0].red, colors[0].green, colors[0].blue],
                [colors[1].red, colors[1].green, colors[1].blue],
            ],
            Colors::RGBTriple(colors) if colors.len() == 2 => [
                [colors[0].red, colors[0].green, colors[0].blue],
                [colors[1].red, colors[1].green, colors[1].blue],
            ],
            _ => return false,
        };
        matches!(
            colors,
            [[0, 0, 0], [255, 255, 255]] | [[255, 255, 255], [0, 0, 0]]
        )
    }

    fn clear_if_unrelated(&mut self, operation: TernaryRasterOperation) {
        if !matches!(
            operation,
            TernaryRasterOperation::PATINVERT | TernaryRasterOperation::DPA
        ) {
            self.state = None;
        }
    }
}

pub struct TernaryRasterOperator {
    operation: TernaryRasterOperation,
    /// [#6617] 장치 좌표로 정규화한 목적 사각형(`DeviceContext::blit_dest_rect`).
    rect: BlitDestRect,
    brush: Option<Brush>,
    source: Option<Source>,
    clip_id: Option<String>,
    clip_rect: Option<[i32; 4]>,
}

enum Source {
    Bitmap16(Bitmap16),
    Bitmap(DeviceIndependentBitmap),
}

impl TernaryRasterOperator {
    pub fn new(operation: TernaryRasterOperation, rect: BlitDestRect) -> Self {
        Self {
            operation,
            rect,
            brush: None,
            source: None,
            clip_id: None,
            clip_rect: None,
        }
    }

    /// 목적 사각형과, 뒤집힌 축이 있으면 그 축을 되돌리는 `transform`.
    ///
    /// [#6140] SVG 의 `width`/`height` 는 음수를 오류로 규정하므로 사각형은 항상 양수 크기로
    /// 두고 뒤집힘을 요소 자신의 `transform` 으로 표현한다. [#6617] 어느 축이 뒤집히는지는
    /// 논리 폭/높이 부호가 아니라 장치 좌표에서 정한다(`DeviceContext::blit_dest_rect`) —
    /// y-up 창의 음수 높이 DIB 는 뒤집히지 않는다.
    fn normalized_rect(&self) -> (i32, i32, i32, i32, Option<String>) {
        let BlitDestRect {
            x,
            y,
            width,
            height,
            flip_x,
            flip_y,
        } = self.rect;
        let mut parts = Vec::new();
        if flip_x {
            parts.push(format!("translate({},0) scale(-1,1)", 2 * x + width));
        }
        if flip_y {
            parts.push(format!("translate(0,{}) scale(1,-1)", 2 * y + height));
        }
        let transform = (!parts.is_empty()).then(|| parts.join(" "));
        (x, y, width, height, transform)
    }

    pub fn brush(mut self, brush: Brush) -> Self {
        self.brush = brush.into();
        self
    }

    pub fn clip(mut self, clip_id: Option<&str>, clip_rect: Option<&Rect>) -> Self {
        self.clip_id = clip_id.map(str::to_owned);
        // INTERSECTCLIPRECT changes the DC without issuing an SVG clip ID.
        // Track both representations before cancelling any raster draw.
        self.clip_rect = clip_rect.map(|rect| {
            [
                i32::from(rect.left),
                i32::from(rect.top),
                i32::from(rect.right),
                i32::from(rect.bottom),
            ]
        });
        self
    }

    pub fn source_bitmap16(mut self, source: Bitmap16) -> Self {
        self.source = Source::Bitmap16(source).into();
        self
    }

    pub fn source_bitmap(mut self, source: DeviceIndependentBitmap) -> Self {
        self.source = Source::Bitmap(source).into();
        self
    }

    pub fn run(
        self,
        definitions: &mut Vec<Node>,
        brush_only_rop_sequence: &mut BrushOnlyRopSequence,
        element_count: usize,
    ) -> Result<Option<Node>, TernaryRasterOperationError> {
        brush_only_rop_sequence.clear_if_unrelated(self.operation);

        if self.operation.use_selected_brush() && self.brush.is_none() {
            return Err(TernaryRasterOperationError::NoBrush {
                cause: format!(
                    "TernaryRasterOperation {:?} cannot access brush.",
                    self.operation,
                ),
            });
        }

        if self.operation.use_source() && self.source.is_none() {
            return Err(TernaryRasterOperationError::NoSource {
                cause: format!(
                    "TernaryRasterOperation {:?} cannot access source bitmap.",
                    self.operation,
                ),
            });
        }

        let result: Node = match self.operation {
            TernaryRasterOperation::BLACKNESS => Node::new("rect")
                .set("x", self.rect.x)
                .set("y", self.rect.y)
                .set("width", self.rect.width)
                .set("height", self.rect.height)
                .set("stroke", "none")
                .set("fill", "black"),
            TernaryRasterOperation::SRCCOPY => {
                let (x, y, width, height, transform) = self.normalized_rect();
                let bitmap = match self.source.unwrap() {
                    Source::Bitmap16(data) => {
                        let bitmap = crate::wmf::parser::DeviceIndependentBitmap::from(data);
                        crate::wmf::converter::Bitmap::from(bitmap)
                    }
                    Source::Bitmap(data) => Bitmap::from(data),
                };

                let image = Node::new("image")
                    .set("x", x)
                    .set("y", y)
                    .set("width", width)
                    .set("height", height)
                    .set("href", bitmap.as_data_url());
                match transform {
                    Some(transform) => image.set("transform", transform),
                    None => image,
                }
            }
            TernaryRasterOperation::PATCOPY => {
                let fill = match Fill::from(self.brush.clone().unwrap()) {
                    Fill::Pattern { pattern } => {
                        let id = Self::issue_id(definitions);
                        definitions.push(pattern.set("id", id.as_str()));
                        url_string(format!("#{id}").as_str())
                    }
                    Fill::Value { value } => value,
                };

                Node::new("rect")
                    .set("x", self.rect.x)
                    .set("y", self.rect.y)
                    .set("width", self.rect.width)
                    .set("height", self.rect.height)
                    .set("fill", fill.as_str())
            }
            TernaryRasterOperation::WHITENESS => Node::new("rect")
                .set("x", self.rect.x)
                .set("y", self.rect.y)
                .set("width", self.rect.width)
                .set("height", self.rect.height)
                .set("stroke", "none")
                .set("fill", "white"),
            // 단독 미구현 브러시 ROP의 기존 PATCOPY 근사는 유지한다.
            // 완전한 PATINVERT → DPa → PATINVERT의 흑백 마스크는 아래
            // 상쇄기가 실제 칠 소속으로 전달한다. 흰 비트는 기존 배경을,
            // 검은 비트는 바깥 브러시 색을 남기므로 패턴을 단색으로 대체하지 않는다.
            // 색상 패턴·불완전한 연산은 이 흑백 마스크 계약으로 해석하지 않는다.
            operation if operation.use_selected_brush() && !operation.use_source() => {
                info!(
                    ?operation,
                    "approximating brush-only TernaryRasterOperation as PATCOPY"
                );
                let is_mask = BrushOnlyRopSequence::brush_is_monochrome_mask(self.brush.as_ref());
                let allow_mask_edge_delta =
                    is_mask && BrushOnlyRopSequence::brush_is_8x8_halftone(self.brush.as_ref());
                let mut has_monochrome_pattern = false;
                let fill = match Fill::from(self.brush.clone().unwrap()) {
                    Fill::Pattern { pattern } => {
                        let id = Self::issue_id(definitions);
                        if is_mask {
                            has_monochrome_pattern = true;
                        }
                        definitions.push(pattern.set("id", id.as_str()));
                        url_string(format!("#{id}").as_str())
                    }
                    Fill::Value { value } => value,
                };

                // `PATINVERT`(D ⊕ P)는 확인된 연속 관용구 안에서만 상쇄한다.
                //
                //   PATINVERT(gray) → DPa(패턴) → PATINVERT(gray)
                //
                // 평면 색 근사로 셋 다 칠하면 마지막 XOR 이 가운데 패턴 blit 이 칠한
                // 그림(흰 원)을 덮는다. 다만 전역 이력에서 같은 키를 찾으면 독립된
                // 후속 draw까지 지워질 수 있으므로, 출력 요소 순서상 연속한
                // PATINVERT → DPA → PATINVERT만 상쇄한다.
                let key = BrushOnlyRopKey {
                    rect: [self.rect.x, self.rect.y, self.rect.width, self.rect.height],
                    fill: fill.clone(),
                    clip_id: self.clip_id,
                    clip_rect: self.clip_rect,
                };
                if brush_only_rop_sequence.observe(
                    operation,
                    key,
                    element_count,
                    has_monochrome_pattern,
                    allow_mask_edge_delta,
                ) {
                    return Ok(None);
                }

                Node::new("rect")
                    .set("x", self.rect.x)
                    .set("y", self.rect.y)
                    .set("width", self.rect.width)
                    .set("height", self.rect.height)
                    .set("fill", fill.as_str())
            }
            operation if operation.use_source() => {
                info!(
                    ?operation,
                    "approximating source TernaryRasterOperation as SRCCOPY"
                );
                let (x, y, width, height, transform) = self.normalized_rect();
                let bitmap = match self.source.unwrap() {
                    Source::Bitmap16(data) => {
                        let bitmap = crate::wmf::parser::DeviceIndependentBitmap::from(data);
                        crate::wmf::converter::Bitmap::from(bitmap)
                    }
                    Source::Bitmap(data) => Bitmap::from(data),
                };

                let image = Node::new("image")
                    .set("x", x)
                    .set("y", y)
                    .set("width", width)
                    .set("height", height)
                    .set("href", bitmap.as_data_url());
                match transform {
                    Some(transform) => image.set("transform", transform),
                    None => image,
                }
            }
            operation => {
                info!(?operation, "TernaryRasterOperation is not implemented");

                return Ok(None);
            }
        };

        Ok(Some(result))
    }

    #[inline]
    fn issue_id(definitions: &[Node]) -> String {
        format!("rop_pat{}", definitions.len())
    }
}

impl From<ColorRef> for RGBQuad {
    fn from(v: ColorRef) -> Self {
        let ColorRef {
            red,
            green,
            blue,
            reserved,
        } = v;
        Self {
            red,
            green,
            blue,
            reserved,
        }
    }
}
