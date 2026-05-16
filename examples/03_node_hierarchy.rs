use spritekit::{CGPoint, Node, NodeExt};

fn main() {
    let parent = Node::new().expect("parent");
    let child = Node::new().expect("child");

    parent.set_name("parent");
    parent.set_user_interaction_enabled(true);
    parent.set_position(CGPoint::new(12.0, 34.0));
    parent.add_child(&child);

    let position = parent.position();
    assert_eq!(parent.name().as_deref(), Some("parent"));
    assert!(parent.is_user_interaction_enabled());
    assert_eq!(parent.children_count(), 1);
    assert!(!parent.has_actions());
    assert!((position.x - 12.0).abs() < 1e-6);
    assert!((position.y - 34.0).abs() < 1e-6);

    child.move_to_parent(&Node::new().expect("other parent"));
    child.remove_from_parent();
    parent.remove_all_children();
    assert_eq!(parent.children_count(), 0);

    println!("node hierarchy ok");
}
