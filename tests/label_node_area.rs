use spritekit::{HorizontalAlignmentMode, LabelNode, VerticalAlignmentMode};

#[test]
fn label_node_round_trips_text_properties() {
    let label = LabelNode::with_font_named("Helvetica").expect("label");
    label.set_text("SpriteKit");
    label.set_font_size(18.0);
    label.set_number_of_lines(3);
    label.set_vertical_alignment_mode(VerticalAlignmentMode::Center);
    label.set_horizontal_alignment_mode(HorizontalAlignmentMode::Right);

    assert_eq!(label.text().as_deref(), Some("SpriteKit"));
    assert!((label.font_size() - 18.0).abs() < f64::EPSILON);
    assert_eq!(label.number_of_lines(), 3);
    assert_eq!(
        label.vertical_alignment_mode(),
        VerticalAlignmentMode::Center
    );
    assert_eq!(
        label.horizontal_alignment_mode(),
        HorizontalAlignmentMode::Right
    );
}
