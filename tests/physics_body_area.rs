use spritekit::{CGPoint, CGVector, PhysicsBody};

#[test]
fn physics_body_round_trips_core_properties() {
    let body = PhysicsBody::circle_at(10.0, CGPoint::new(1.0, 2.0)).expect("body");
    body.set_dynamic(false);
    body.set_pinned(true);
    body.set_charge(2.0);
    body.set_velocity(CGVector::new(5.0, 6.0));

    let velocity = body.velocity();
    assert!(!body.is_dynamic());
    assert!(body.is_pinned());
    assert!((body.charge() - 2.0).abs() < 1e-6);
    assert!((velocity.dx - 5.0).abs() < 1e-6);
    assert!((velocity.dy - 6.0).abs() < 1e-6);
    assert!(body.area() > 0.0);
}
