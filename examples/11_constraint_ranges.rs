use spritekit::{CGPoint, Constraint, ConstraintRange, Node, NodeExt};

fn main() {
    let subject = Node::new().expect("subject");
    let reference = Node::new().expect("reference");
    let x_range = ConstraintRange::with_limits(-10.0, 10.0).expect("x range");
    let y_range = ConstraintRange::with_constant(4.0).expect("y range");
    let orient_offset = ConstraintRange::with_constant(0.0).expect("offset range");

    let position = Constraint::position_xy(&x_range, &y_range).expect("position constraint");
    position.set_enabled(true);
    position.set_reference_node(Some(&reference));

    let orient = Constraint::orient_to_point(CGPoint::new(2.0, 1.0), &orient_offset)
        .expect("orient constraint");
    subject.set_constraints(&[&position, &orient]);

    assert_eq!(subject.constraint_count(), 2);
    assert!(position.is_enabled());
    assert!(position.reference_node().is_some());
    assert!((x_range.lower_limit() + 10.0).abs() < 1e-6);
    assert!((y_range.upper_limit() - 4.0).abs() < 1e-6);

    println!("constraint ranges ok");
}
