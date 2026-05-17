use spritekit::{
    CGPoint, CGVector, PhysicsBody, PhysicsJointExt, PhysicsJointPin, PhysicsJointSliding,
    PhysicsJointSpring,
};

#[test]
fn physics_joints_expose_basic_properties() {
    let body_a = PhysicsBody::circle(5.0).expect("body a");
    let body_b = PhysicsBody::rect(8.0, 4.0).expect("body b");

    let pin = PhysicsJointPin::new(&body_a, &body_b, CGPoint::new(0.0, 0.0)).expect("pin");
    pin.set_friction_torque(1.25);
    pin.set_should_enable_limits(true);

    let spring = PhysicsJointSpring::new(
        &body_a,
        &body_b,
        CGPoint::new(-1.0, 0.0),
        CGPoint::new(1.0, 0.0),
    )
    .expect("spring");
    spring.set_frequency(3.0);

    let sliding = PhysicsJointSliding::new(
        &body_a,
        &body_b,
        CGPoint::new(0.0, 0.0),
        CGVector::new(1.0, 0.0),
    )
    .expect("sliding");
    sliding.set_upper_distance_limit(2.5);

    assert!(pin.friction_torque().is_finite());
    assert!(spring.frequency().is_finite());
    assert!(sliding.upper_distance_limit().is_finite());
    assert!(pin.reaction_force().dx.is_finite());
    assert!(pin.reaction_torque().is_finite());
}
