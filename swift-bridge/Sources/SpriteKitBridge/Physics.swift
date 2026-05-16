import SpriteKit

// MARK: - PhysicsBody constructors

@_cdecl("sk_physics_body_circle")
public func sk_physics_body_circle(_ radius: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKPhysicsBody(circleOfRadius: CGFloat(radius)))
}

@_cdecl("sk_physics_body_rect")
public func sk_physics_body_rect(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKPhysicsBody(rectangleOf: CGSize(width: width, height: height)))
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

// MARK: - dynamic / simulation flags

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

// MARK: - material properties

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

@_cdecl("sk_physics_body_get_gravity_scale")
public func sk_physics_body_get_gravity_scale(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return 1 }
    return Double(body.affectedByGravity ? 1.0 : 0.0)
}

@_cdecl("sk_physics_body_set_affected_by_gravity")
public func sk_physics_body_set_affected_by_gravity(_ bodyHandle: UnsafeMutableRawPointer?, _ affected: Bool) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.affectedByGravity = affected
}

// MARK: - collision masks

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

// MARK: - velocity

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

// MARK: - force / impulse

@_cdecl("sk_physics_body_apply_force")
public func sk_physics_body_apply_force(_ bodyHandle: UnsafeMutableRawPointer?, _ dx: Double, _ dy: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyForce(CGVector(dx: dx, dy: dy))
}

@_cdecl("sk_physics_body_apply_impulse")
public func sk_physics_body_apply_impulse(_ bodyHandle: UnsafeMutableRawPointer?, _ dx: Double, _ dy: Double) {
    guard let body: SKPhysicsBody = skBorrow(bodyHandle) else { return }
    body.applyImpulse(CGVector(dx: dx, dy: dy))
}

// MARK: - PhysicsWorld

@_cdecl("sk_physics_world_get_gravity_dx")
public func sk_physics_world_get_gravity_dx(_ worldHandle: UnsafeMutableRawPointer?) -> Double {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return 0 }
    return Double(world.gravity.dx)
}

@_cdecl("sk_physics_world_get_gravity_dy")
public func sk_physics_world_get_gravity_dy(_ worldHandle: UnsafeMutableRawPointer?) -> Double {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return -9.8 }
    return Double(world.gravity.dy)
}

@_cdecl("sk_physics_world_set_gravity")
public func sk_physics_world_set_gravity(_ worldHandle: UnsafeMutableRawPointer?, _ dx: Double, _ dy: Double) {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return }
    world.gravity = CGVector(dx: dx, dy: dy)
}

@_cdecl("sk_physics_world_get_speed")
public func sk_physics_world_get_speed(_ worldHandle: UnsafeMutableRawPointer?) -> Double {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return 1 }
    return Double(world.speed)
}

@_cdecl("sk_physics_world_set_speed")
public func sk_physics_world_set_speed(_ worldHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return }
    world.speed = CGFloat(speed)
}
