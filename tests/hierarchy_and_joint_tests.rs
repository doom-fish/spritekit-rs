use spritekit::{
    CGPoint, CGSize, Node, NodeExt, PhysicsBody, PhysicsJointFixed, PhysicsJointPin, Scene,
};

fn node_with_body(scene: Option<&Scene>, x: f64) -> (Node, PhysicsBody) {
    let node = Node::new().expect("node");
    node.set_position(CGPoint::new(x, 32.0));
    let body = PhysicsBody::circle(4.0).expect("body");
    node.set_physics_body(Some(&body));
    if let Some(scene) = scene {
        scene.add_child(&node).expect("add body node");
    }
    (node, body)
}

#[test]
fn add_child_rejects_children_that_already_have_a_parent() {
    let first = Node::new().expect("first");
    let second = Node::new().expect("second");
    let child = Node::new().expect("child");
    first.add_child(&child).expect("first add");

    let error = second
        .add_child(&child)
        .expect_err("a parented child must be rejected");
    assert!(
        error.to_string().contains("already has a parent"),
        "{error}"
    );
    assert_eq!(first.children_count(), 1);
    assert_eq!(second.children_count(), 0);

    child.remove_from_parent();
    second.add_child(&child).expect("add after removal");
    assert_eq!(first.children_count(), 0);
    assert_eq!(second.children_count(), 1);
}

#[test]
fn add_child_rejects_self_and_ancestors() {
    let root = Node::new().expect("root");
    let middle = Node::new().expect("middle");
    let leaf = Node::new().expect("leaf");
    root.add_child(&middle).expect("root -> middle");
    middle.add_child(&leaf).expect("middle -> leaf");

    let lone = Node::new().expect("lone");
    let error = lone.add_child(&lone).expect_err("self must be rejected");
    assert!(error.to_string().contains("itself"), "{error}");
    assert_eq!(lone.children_count(), 0);

    let error = leaf
        .add_child(&root)
        .expect_err("an ancestor must be rejected");
    assert!(error.to_string().contains("descendant"), "{error}");
    assert_eq!(root.children_count(), 1);
    assert_eq!(middle.children_count(), 1);
    assert_eq!(leaf.children_count(), 0);
}

#[test]
fn move_to_parent_reparents_and_rejects_cycles() {
    let scene = Scene::with_size(CGSize::new(32.0, 32.0)).expect("scene");
    let first = Node::new().expect("first");
    let second = Node::new().expect("second");
    let child = Node::new().expect("child");
    scene.add_child(&first).expect("scene -> first");
    scene.add_child(&second).expect("scene -> second");
    first.add_child(&child).expect("first -> child");

    child.move_to_parent(&second).expect("move child");
    assert_eq!(first.children_count(), 0);
    assert_eq!(second.children_count(), 1);

    let error = second
        .move_to_parent(&child)
        .expect_err("moving under a descendant must be rejected");
    assert!(error.to_string().contains("descendants"), "{error}");
    let error = child
        .move_to_parent(&child)
        .expect_err("moving under itself must be rejected");
    assert!(error.to_string().contains("itself"), "{error}");
    assert_eq!(second.children_count(), 1);
    assert_eq!(child.children_count(), 0);
}

#[test]
fn joints_between_scene_bodies_can_be_added_and_removed() {
    let scene = Scene::with_size(CGSize::new(64.0, 64.0)).expect("scene");
    let world = scene.physics_world();
    let (_node_a, body_a) = node_with_body(Some(&scene), 20.0);
    let (_node_b, body_b) = node_with_body(Some(&scene), 40.0);
    let joint = PhysicsJointPin::new(&body_a, &body_b, CGPoint::new(30.0, 32.0)).expect("pin");

    world.remove_joint(&joint);
    world.add_joint(&joint).expect("add joint");
    let error = world
        .add_joint(&joint)
        .expect_err("adding the same joint twice must be rejected");
    assert!(error.to_string().contains("already"), "{error}");

    world.remove_joint(&joint);
    world.add_joint(&joint).expect("re-add after removal");
    world.remove_all_joints();
    world
        .add_joint(&joint)
        .expect("re-add after removing all joints");
    world.remove_all_joints();
}

#[test]
fn joints_with_bodies_outside_a_scene_are_rejected() {
    let scene = Scene::with_size(CGSize::new(64.0, 64.0)).expect("scene");
    let world = scene.physics_world();
    let (_attached_node, attached) = node_with_body(Some(&scene), 20.0);
    let (_detached_node, detached) = node_with_body(None, 40.0);
    let loose = PhysicsBody::circle(4.0).expect("loose body");

    for (body_a, body_b) in [
        (&attached, &detached),
        (&attached, &loose),
        (&loose, &detached),
    ] {
        let joint =
            PhysicsJointFixed::new(body_a, body_b, CGPoint::new(30.0, 32.0)).expect("fixed");
        let error = world
            .add_joint(&joint)
            .expect_err("bodies outside a scene must be rejected");
        assert!(
            error.to_string().contains("attached to nodes in a scene"),
            "{error}"
        );
    }
}
