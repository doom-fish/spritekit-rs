use spritekit::{CGPoint, CGSize, NodeExt, PhysicsBody, Scene};

fn main() {
    let scene = Scene::with_size(CGSize::new(128.0, 128.0)).expect("scene");
    let node =
        spritekit::SpriteNode::with_color(spritekit::Color::green(), CGSize::new(16.0, 16.0))
            .expect("sprite");
    node.set_position(CGPoint::new(0.0, 0.0));
    node.set_physics_body(PhysicsBody::circle(8.0).as_ref());
    scene.add_child(&node);

    let world = scene.physics_world();
    assert!(world.body_at_point(CGPoint::new(0.0, 0.0)).is_some());
    assert!(world
        .body_in_rect(spritekit::CGRect::new(-10.0, -10.0, 20.0, 20.0))
        .is_some());
    assert!(world
        .body_along_ray(CGPoint::new(-20.0, 0.0), CGPoint::new(20.0, 0.0))
        .is_some());

    println!("physics world queries ok");
}
