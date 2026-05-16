use spritekit::{HorizontalAlignmentMode, LabelNode, VerticalAlignmentMode};

fn main() {
    let label = LabelNode::with_font_named("Helvetica").expect("label");
    label.set_text("Hello SpriteKit");
    label.set_font_size(24.0);
    label.set_number_of_lines(2);
    label.set_preferred_max_layout_width(200.0);
    label.set_vertical_alignment_mode(VerticalAlignmentMode::Top);
    label.set_horizontal_alignment_mode(HorizontalAlignmentMode::Left);
    label.set_color_blend_factor(0.4);

    assert_eq!(label.text().as_deref(), Some("Hello SpriteKit"));
    assert!((label.font_size() - 24.0).abs() < f64::EPSILON);
    assert_eq!(label.number_of_lines(), 2);
    assert_eq!(label.vertical_alignment_mode(), VerticalAlignmentMode::Top);
    assert_eq!(label.horizontal_alignment_mode(), HorizontalAlignmentMode::Left);

    println!("label node text ok");
}
