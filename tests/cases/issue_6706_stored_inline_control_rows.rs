//! #6706: 같은 가시 문자 위치에 투영된 표와 그림도 원시 UTF-16 저장 줄은 다르다.
//! 실물 HWPX와 동일 입력의 한컴 PDF `pdf/hwp3-sample16-hwp5-hwpx-2024.pdf` 18쪽 대조.
//! 제목 표 → 그림 → 뒤 제목 표의 소유·포함·순서를 검사한다.
//! 절대 픽셀은 고정하지 않는다. 같은 줄의 그림+표 반례는 #6754가 지킨다.
#![cfg(not(target_arch = "wasm32"))]

use rhwp::document_core::DocumentCore;
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};

fn collect<'a>(node: &'a RenderNode, nodes: &mut Vec<&'a RenderNode>) {
    nodes.push(node);
    for child in &node.children {
        collect(child, nodes);
    }
}

fn owned_title<'a>(nodes: &[&'a RenderNode], control: usize, text: &str) -> &'a RenderNode {
    let titles: Vec<_> = nodes
        .iter()
        .copied()
        .filter(|node| matches!(&node.node_type, RenderNodeType::TextRun(run) if run.text == text))
        .collect();
    assert_eq!(titles.len(), 1, "제목을 빠뜨리거나 중복 표시하지 않는다");
    let title = titles[0];
    let RenderNodeType::TextRun(run) = &title.node_type else {
        unreachable!()
    };
    let context = run
        .cell_context
        .as_ref()
        .expect("제목은 원본 표 셀을 소유한다");
    assert_eq!(context.parent_para_index, 394);
    assert_eq!(context.path.len(), 1);
    assert_eq!(context.path[0].control_index, control);
    assert_eq!(context.path[0].cell_index, 0);
    let tables: Vec<_> = nodes
        .iter()
        .copied()
        .filter(|node| {
            matches!(&node.node_type, RenderNodeType::Table(table)
            if table.para_index == Some(394) && table.control_index == Some(control))
        })
        .collect();
    assert_eq!(tables.len(), 1, "제목 표를 한 번만 배치한다");
    let table = tables[0];
    assert!(
        table.bbox.contains(&title.bbox),
        "제목은 소유 표 내부에 표시한다"
    );
    let mut descendants = Vec::new();
    collect(table, &mut descendants);
    assert!(
        descendants.iter().any(|cell| {
            if !matches!(cell.node_type, RenderNodeType::TableCell(_)) {
                return false;
            }
            let mut children = Vec::new();
            collect(cell, &mut children);
            children.iter().any(|child| child.id == title.id) && cell.bbox.contains(&title.bbox)
        }),
        "제목은 실제 소유 셀의 자식이며 그 내부에 표시한다"
    );
    table
}

#[test]
fn stored_table_picture_table_rows_keep_hancom_positions_and_unique_ownership() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("samples/hwp3-sample16-hwp5.hwpx");
    let core = DocumentCore::from_bytes(&std::fs::read(path).unwrap()).unwrap();
    assert_eq!(core.page_count(), 64, "독립 한컴 Print PDF의 전체 쪽수");
    let page = core.build_page_render_tree(17).unwrap();
    let mut nodes = Vec::new();
    collect(&page.root, &mut nodes);
    let before = owned_title(&nodes, 0, "가. 주전산센터 목표시스템 구성(안)");
    let after = owned_title(&nodes, 2, "나. 주요 과업내용");

    let pictures: Vec<_> = nodes
        .iter()
        .filter(|n| {
            matches!(&n.node_type, RenderNodeType::Image(i)
            if i.para_index == Some(394) && i.control_index == Some(1))
        })
        .collect();
    assert_eq!(
        pictures.len(),
        1,
        "그림은 원시 둘째 줄에 한 번만 배치해야 한다"
    );
    let picture = pictures[0];
    assert!(
        page.root.bbox.contains(&picture.bbox),
        "그림은 쪽 내부에 표시한다"
    );
    assert!(
        before.bbox.y + before.bbox.height < picture.bbox.y,
        "그림은 첫 제목 표 뒤에 있다"
    );
    assert!(
        picture.bbox.y + picture.bbox.height < after.bbox.y,
        "뒤 제목 표는 그림 아래에 있다"
    );
}
