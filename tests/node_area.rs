use spritekit::{CGPoint, Node, NodeExt};

#[test]
fn node_tracks_children_and_position() {
    let parent = Node::new().expect("parent");
    let child = Node::new().expect("child");

    parent.set_name("parent-node");
    parent.set_position(CGPoint::new(3.0, 7.0));
    parent.set_user_interaction_enabled(true);
    parent.add_child(&child);

    let position = parent.position();
    assert_eq!(parent.name().as_deref(), Some("parent-node"));
    assert!((position.x - 3.0).abs() < 1e-6);
    assert!((position.y - 7.0).abs() < 1e-6);
    assert!(parent.is_user_interaction_enabled());
    assert_eq!(parent.children_count(), 1);
}
