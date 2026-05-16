use spritekit::{CGPoint, CGVector, PhysicsBody};

fn main() {
    let primary = PhysicsBody::circle_at(12.0, CGPoint::new(1.0, 2.0)).expect("circle body");
    let secondary = PhysicsBody::rect(8.0, 4.0).expect("rect body");
    let compound = PhysicsBody::compound(&[&primary, &secondary]).expect("compound body");

    compound.set_dynamic(false);
    compound.set_pinned(true);
    compound.set_charge(1.5);
    compound.set_velocity(CGVector::new(3.0, 4.0));
    compound.apply_force(CGVector::new(1.0, 0.0));
    compound.apply_impulse_at_point(CGVector::new(0.5, 0.25), CGPoint::new(0.0, 0.0));

    let velocity = compound.velocity();
    assert!(!compound.is_dynamic());
    assert!(compound.is_pinned());
    assert!((compound.charge() - 1.5).abs() < 1e-6);
    assert!((velocity.dx - 3.0).abs() < 1e-6);
    assert!((velocity.dy - 4.0).abs() < 1e-6);
    assert!(compound.area() > 0.0);

    println!("physics body properties ok");
}
