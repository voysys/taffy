pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(0f32) },
            max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(100f32) },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(320f32), height: auto() },
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(0f32) },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
