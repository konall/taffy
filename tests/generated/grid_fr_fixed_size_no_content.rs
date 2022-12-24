#[test]
fn grid_fr_fixed_size_no_content() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32), flex(1f32), flex(1f32)],
                grid_template_columns: vec![points(40f32), flex(1f32), flex(1f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 80f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 80f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 120f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3.data(), 40f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node3.data(), 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3.data(), 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node4.data(), 80f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node4.data(), 80f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node4.data(), 40f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node4.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node5.data(), 80f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node5.data(), 80f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node5.data(), 120f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node5.data(), 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6.data(), 40f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node6.data(), 80f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node6.data(), 0f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node6.data(), 120f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node7).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node7.data(), 80f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node7.data(), 80f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node7.data(), 40f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node7.data(), 120f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node8).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node8.data(), 80f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node8.data(), 80f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node8.data(), 120f32, location.x);
    assert_eq!(location.y, 120f32, "y of node {:?}. Expected {}. Actual {}", node8.data(), 120f32, location.y);
}
