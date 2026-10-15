//! BorderFill의 paint 결과. 문단과 셀이 같은 색·무늬·그라데이션·그림 의미를 쓴다.

use super::*;
use crate::renderer::style_resolver::ResolvedBorderStyle;

impl LayoutEngine {
    pub(super) fn border_fill_background_nodes(
        tree: &mut PageLayoutContext,
        style: Option<&ResolvedBorderStyle>,
        bounds: BoundingBox,
        bin_data_content: &[BinDataContent],
    ) -> Vec<RenderNode> {
        let mut nodes = Vec::new();
        let fill_color = style.and_then(|s| s.fill_color);
        let pattern = style.and_then(|s| s.pattern);
        let gradient = style.and_then(|s| s.gradient.clone());
        if fill_color.is_some() || pattern.is_some() || gradient.is_some() {
            nodes.push(RenderNode::new(
                tree.next_id(),
                RenderNodeType::Rectangle(RectangleNode::new(
                    0.0,
                    ShapeStyle {
                        fill_color,
                        pattern,
                        ..Default::default()
                    },
                    gradient,
                )),
                bounds,
            ));
        }
        if let Some(fill) = style.and_then(|s| s.image_fill.as_ref()) {
            if let Some(bytes) = find_bin_data_bytes(bin_data_content, fill.bin_data_id) {
                // ResolvedImageFill은 이진 순서, ImageNode는 화면 순서다(#6895).
                let (brightness, contrast) = fill.display_brightness_contrast();
                nodes.push(RenderNode::new(
                    tree.next_id(),
                    RenderNodeType::Image(ImageNode {
                        fill_mode: Some(fill.fill_mode),
                        brightness,
                        contrast,
                        effect: fill.effect,
                        ..ImageNode::new(fill.bin_data_id, Some(bytes))
                    }),
                    bounds,
                ));
            }
        }
        nodes
    }
}
