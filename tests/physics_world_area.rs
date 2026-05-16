use spritekit::{CGPoint, CGRect, CGSize, NodeExt, PhysicsBody, Scene, SpriteNode};

#[test]
fn physics_world_can_find_attached_bodies() {
    let scene = Scene::with_size(CGSize::new(100.0, 100.0)).expect("scene");
    let node = SpriteNode::with_color(spritekit::Color::green(), CGSize::new(10.0, 10.0)).expect("node");
    let body = PhysicsBody::circle(6.0).expect("body");

    node.set_position(CGPoint::new(0.0, 0.0));
    node.set_physics_body(Some(&body));
    scene.add_child(&node);

    let world = scene.physics_world();
    assert!(world.body_at_point(CGPoint::new(0.0, 0.0)).is_some());
    assert!(world.body_in_rect(CGRect::new(-8.0, -8.0, 16.0, 16.0)).is_some());
    assert!(world
        .body_along_ray(CGPoint::new(-20.0, 0.0), CGPoint::new(20.0, 0.0))
        .is_some());
}
