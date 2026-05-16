use spritekit::{Constraint, ConstraintRange, Node, NodeExt};

#[test]
fn constraints_attach_to_nodes() {
    let node = Node::new().expect("node");
    let reference = Node::new().expect("reference");
    let range = ConstraintRange::with_limits(-5.0, 5.0).expect("range");
    let constraint = Constraint::position_x(&range).expect("constraint");

    constraint.set_reference_node(Some(&reference));
    node.set_constraints(&[&constraint]);

    assert_eq!(node.constraint_count(), 1);
    assert!(constraint.reference_node().is_some());
    assert!(constraint.is_enabled());
    assert!((range.lower_limit() + 5.0).abs() < 1e-6);
    assert!((range.upper_limit() - 5.0).abs() < 1e-6);
}
