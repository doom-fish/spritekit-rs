use spritekit::{CGPoint, CGVector, PhysicsBody, PhysicsJointExt, PhysicsJointFixed, PhysicsJointPin, PhysicsJointSliding, PhysicsJointSpring};

fn main() {
    let body_a = PhysicsBody::circle(8.0).expect("body a");
    let body_b = PhysicsBody::rect(10.0, 6.0).expect("body b");

    let pin = PhysicsJointPin::new(&body_a, &body_b, CGPoint::new(0.0, 0.0)).expect("pin joint");
    pin.set_should_enable_limits(true);
    pin.set_lower_angle_limit(-0.5);
    pin.set_upper_angle_limit(0.5);
    pin.set_friction_torque(2.0);
    pin.set_rotation_speed(1.25);

    let spring = PhysicsJointSpring::new(
        &body_a,
        &body_b,
        CGPoint::new(-2.0, 0.0),
        CGPoint::new(2.0, 0.0),
    )
    .expect("spring joint");
    spring.set_damping(0.75);
    spring.set_frequency(4.0);

    let _fixed = PhysicsJointFixed::new(&body_a, &body_b, CGPoint::new(0.0, 0.0)).expect("fixed joint");
    let sliding = PhysicsJointSliding::new(
        &body_a,
        &body_b,
        CGPoint::new(0.0, 0.0),
        CGVector::new(1.0, 0.0),
    )
    .expect("sliding joint");
    sliding.set_should_enable_limits(true);
    sliding.set_lower_distance_limit(-3.0);
    sliding.set_upper_distance_limit(3.0);

    assert!(pin.friction_torque().is_finite());
    assert!(spring.damping().is_finite());
    assert!(spring.frequency().is_finite());
    assert!(sliding.upper_distance_limit().is_finite());
    assert!(pin.reaction_force().dx.is_finite());

    println!("physics joint basics ok");
}
