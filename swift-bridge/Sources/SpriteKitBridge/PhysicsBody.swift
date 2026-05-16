import SpriteKit

@_cdecl("sk_physics_body_circle")
public func sk_physics_body_circle(_ radius: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKPhysicsBody(circleOfRadius: CGFloat(radius)))
}

@_cdecl("sk_physics_body_circle_center")
public func sk_physics_body_circle_center(_ radius: Double, _ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKPhysicsBody(circleOfRadius: CGFloat(radius), center: CGPoint(x: x, y: y)))
}

@_cdecl("sk_physics_body_rect")
public func sk_physics_body_rect(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKPhysicsBody(rectangleOf: CGSize(width: width, height: height)))
}

@_cdecl("sk_physics_body_rect_center")
public func sk_physics_body_rect_center(_ width: Double, _ height: Double, _ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKPhysicsBody(rectangleOf: CGSize(width: width, height: height), center: CGPoint(x: x, y: y)))
}

@_cdecl("sk_physics_body_edge_loop_rect")
public func sk_physics_body_edge_loop_rect(
    _ x: Double, _ y: Double, _ width: Double, _ height: Double
) -> UnsafeMutableRawPointer? {
    skRetain(SKPhysicsBody(edgeLoopFrom: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("sk_physics_body_texture")
public func sk_physics_body_texture(
    _ textureHandle: UnsafeMutableRawPointer?,
    _ sizeW: Double, _ sizeH: Double
) -> UnsafeMutableRawPointer? {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return nil }
    return skRetain(SKPhysicsBody(texture: texture, size: CGSize(width: sizeW, height: sizeH)))
}

@_cdecl("sk_physics_body_compound")
public func sk_physics_body_compound(_ rawBodies: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    let bodies = skBodies(from: rawBodies, count: count)
    guard !bodies.isEmpty else { return nil }
    return skRetain(SKPhysicsBody(bodies: bodies))
}

@_cdecl("sk_physics_body_get_dynamic")
public func sk_physics_body_get_dynamic(_ bodyHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return false }
    return body.isDynamic
}

@_cdecl("sk_physics_body_set_dynamic")
public func sk_physics_body_set_dynamic(_ bodyHandle: UnsafeMutableRawPointer?, _ dynamic: Bool) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.isDynamic = dynamic
}

@_cdecl("sk_physics_body_get_allows_rotation")
public func sk_physics_body_get_allows_rotation(_ bodyHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return true }
    return body.allowsRotation
}

@_cdecl("sk_physics_body_set_allows_rotation")
public func sk_physics_body_set_allows_rotation(_ bodyHandle: UnsafeMutableRawPointer?, _ allows: Bool) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.allowsRotation = allows
}

@_cdecl("sk_physics_body_get_precise_collision")
public func sk_physics_body_get_precise_collision(_ bodyHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return false }
    return body.usesPreciseCollisionDetection
}

@_cdecl("sk_physics_body_set_precise_collision")
public func sk_physics_body_set_precise_collision(_ bodyHandle: UnsafeMutableRawPointer?, _ precise: Bool) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.usesPreciseCollisionDetection = precise
}

@_cdecl("sk_physics_body_get_pinned")
public func sk_physics_body_get_pinned(_ bodyHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return false }
    return body.pinned
}

@_cdecl("sk_physics_body_set_pinned")
public func sk_physics_body_set_pinned(_ bodyHandle: UnsafeMutableRawPointer?, _ pinned: Bool) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.pinned = pinned
}

@_cdecl("sk_physics_body_get_resting")
public func sk_physics_body_get_resting(_ bodyHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return false }
    return body.isResting
}

@_cdecl("sk_physics_body_set_resting")
public func sk_physics_body_set_resting(_ bodyHandle: UnsafeMutableRawPointer?, _ resting: Bool) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.isResting = resting
}

@_cdecl("sk_physics_body_get_friction")
public func sk_physics_body_get_friction(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return Double(body.friction)
}

@_cdecl("sk_physics_body_set_friction")
public func sk_physics_body_set_friction(_ bodyHandle: UnsafeMutableRawPointer?, _ friction: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.friction = CGFloat(friction)
}

@_cdecl("sk_physics_body_get_charge")
public func sk_physics_body_get_charge(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return Double(body.charge)
}

@_cdecl("sk_physics_body_set_charge")
public func sk_physics_body_set_charge(_ bodyHandle: UnsafeMutableRawPointer?, _ charge: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.charge = CGFloat(charge)
}

@_cdecl("sk_physics_body_get_restitution")
public func sk_physics_body_get_restitution(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0.2 }
    return Double(body.restitution)
}

@_cdecl("sk_physics_body_set_restitution")
public func sk_physics_body_set_restitution(_ bodyHandle: UnsafeMutableRawPointer?, _ restitution: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.restitution = CGFloat(restitution)
}

@_cdecl("sk_physics_body_get_linear_damping")
public func sk_physics_body_get_linear_damping(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0.1 }
    return Double(body.linearDamping)
}

@_cdecl("sk_physics_body_set_linear_damping")
public func sk_physics_body_set_linear_damping(_ bodyHandle: UnsafeMutableRawPointer?, _ damping: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.linearDamping = CGFloat(damping)
}

@_cdecl("sk_physics_body_get_angular_damping")
public func sk_physics_body_get_angular_damping(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0.1 }
    return Double(body.angularDamping)
}

@_cdecl("sk_physics_body_set_angular_damping")
public func sk_physics_body_set_angular_damping(_ bodyHandle: UnsafeMutableRawPointer?, _ damping: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.angularDamping = CGFloat(damping)
}

@_cdecl("sk_physics_body_get_density")
public func sk_physics_body_get_density(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 1 }
    return Double(body.density)
}

@_cdecl("sk_physics_body_set_density")
public func sk_physics_body_set_density(_ bodyHandle: UnsafeMutableRawPointer?, _ density: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.density = CGFloat(density)
}

@_cdecl("sk_physics_body_get_mass")
public func sk_physics_body_get_mass(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return Double(body.mass)
}

@_cdecl("sk_physics_body_set_mass")
public func sk_physics_body_set_mass(_ bodyHandle: UnsafeMutableRawPointer?, _ mass: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.mass = CGFloat(mass)
}

@_cdecl("sk_physics_body_get_area")
public func sk_physics_body_get_area(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return Double(body.area)
}

@_cdecl("sk_physics_body_get_affected_by_gravity")
public func sk_physics_body_get_affected_by_gravity(_ bodyHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return true }
    return body.affectedByGravity
}

@_cdecl("sk_physics_body_set_affected_by_gravity")
public func sk_physics_body_set_affected_by_gravity(_ bodyHandle: UnsafeMutableRawPointer?, _ affected: Bool) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.affectedByGravity = affected
}

@_cdecl("sk_physics_body_get_field_bitmask")
public func sk_physics_body_get_field_bitmask(_ bodyHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return body.fieldBitMask
}

@_cdecl("sk_physics_body_set_field_bitmask")
public func sk_physics_body_set_field_bitmask(_ bodyHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.fieldBitMask = mask
}

@_cdecl("sk_physics_body_get_category_bitmask")
public func sk_physics_body_get_category_bitmask(_ bodyHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return body.categoryBitMask
}

@_cdecl("sk_physics_body_set_category_bitmask")
public func sk_physics_body_set_category_bitmask(_ bodyHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.categoryBitMask = mask
}

@_cdecl("sk_physics_body_get_contact_test_bitmask")
public func sk_physics_body_get_contact_test_bitmask(_ bodyHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return body.contactTestBitMask
}

@_cdecl("sk_physics_body_set_contact_test_bitmask")
public func sk_physics_body_set_contact_test_bitmask(_ bodyHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.contactTestBitMask = mask
}

@_cdecl("sk_physics_body_get_collision_bitmask")
public func sk_physics_body_get_collision_bitmask(_ bodyHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return body.collisionBitMask
}

@_cdecl("sk_physics_body_set_collision_bitmask")
public func sk_physics_body_set_collision_bitmask(_ bodyHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.collisionBitMask = mask
}

@_cdecl("sk_physics_body_get_velocity_dx")
public func sk_physics_body_get_velocity_dx(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return Double(body.velocity.dx)
}

@_cdecl("sk_physics_body_get_velocity_dy")
public func sk_physics_body_get_velocity_dy(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return Double(body.velocity.dy)
}

@_cdecl("sk_physics_body_set_velocity")
public func sk_physics_body_set_velocity(_ bodyHandle: UnsafeMutableRawPointer?, _ dx: Double, _ dy: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.velocity = CGVector(dx: dx, dy: dy)
}

@_cdecl("sk_physics_body_get_angular_velocity")
public func sk_physics_body_get_angular_velocity(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return Double(body.angularVelocity)
}

@_cdecl("sk_physics_body_set_angular_velocity")
public func sk_physics_body_set_angular_velocity(_ bodyHandle: UnsafeMutableRawPointer?, _ velocity: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.angularVelocity = CGFloat(velocity)
}

@_cdecl("sk_physics_body_apply_force")
public func sk_physics_body_apply_force(_ bodyHandle: UnsafeMutableRawPointer?, _ dx: Double, _ dy: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyForce(CGVector(dx: dx, dy: dy))
}

@_cdecl("sk_physics_body_apply_force_at_point")
public func sk_physics_body_apply_force_at_point(
    _ bodyHandle: UnsafeMutableRawPointer?,
    _ forceDx: Double, _ forceDy: Double,
    _ pointX: Double, _ pointY: Double
) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyForce(CGVector(dx: forceDx, dy: forceDy), at: CGPoint(x: pointX, y: pointY))
}

@_cdecl("sk_physics_body_apply_torque")
public func sk_physics_body_apply_torque(_ bodyHandle: UnsafeMutableRawPointer?, _ torque: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyTorque(CGFloat(torque))
}

@_cdecl("sk_physics_body_apply_impulse")
public func sk_physics_body_apply_impulse(_ bodyHandle: UnsafeMutableRawPointer?, _ dx: Double, _ dy: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyImpulse(CGVector(dx: dx, dy: dy))
}

@_cdecl("sk_physics_body_apply_impulse_at_point")
public func sk_physics_body_apply_impulse_at_point(
    _ bodyHandle: UnsafeMutableRawPointer?,
    _ impulseDx: Double, _ impulseDy: Double,
    _ pointX: Double, _ pointY: Double
) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyImpulse(CGVector(dx: impulseDx, dy: impulseDy), at: CGPoint(x: pointX, y: pointY))
}

@_cdecl("sk_physics_body_apply_angular_impulse")
public func sk_physics_body_apply_angular_impulse(_ bodyHandle: UnsafeMutableRawPointer?, _ impulse: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyAngularImpulse(CGFloat(impulse))
}

@_cdecl("sk_physics_body_all_contacted_bodies_count")
public func sk_physics_body_all_contacted_bodies_count(_ bodyHandle: UnsafeMutableRawPointer?) -> Int {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 0 }
    return body.allContactedBodies().count
}
