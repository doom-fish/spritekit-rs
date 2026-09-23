import SpriteKit

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

@_cdecl("sk_physics_world_body_at_point")
public func sk_physics_world_body_at_point(_ worldHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle), let body = world.body(at: CGPoint(x: x, y: y)) else { return nil }
    return skRetain(body)
}

@_cdecl("sk_physics_world_body_in_rect")
public func sk_physics_world_body_in_rect(_ worldHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle), let body = world.body(in: CGRect(x: x, y: y, width: width, height: height)) else { return nil }
    return skRetain(body)
}

@_cdecl("sk_physics_world_body_along_ray")
public func sk_physics_world_body_along_ray(_ worldHandle: UnsafeMutableRawPointer?, _ startX: Double, _ startY: Double, _ endX: Double, _ endY: Double) -> UnsafeMutableRawPointer? {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle), let body = world.body(alongRayStart: CGPoint(x: startX, y: startY), end: CGPoint(x: endX, y: endY)) else { return nil }
    return skRetain(body)
}

@_cdecl("sk_physics_world_add_joint")
public func sk_physics_world_add_joint(
    _ worldHandle: UnsafeMutableRawPointer?,
    _ jointHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    outError?.pointee = nil
    guard let world: SKPhysicsWorld = skBorrow(worldHandle), let joint: SKPhysicsJoint = skBorrow(jointHandle) else {
        outError?.pointee = skDup("missing physics world or joint")
        return false
    }
    guard joint.bodyA.node?.scene != nil, joint.bodyB.node?.scene != nil else {
        outError?.pointee = skDup("both joint bodies must be attached to nodes in a scene")
        return false
    }
    guard !joint.bodyA.joints.contains(where: { $0 === joint }) else {
        outError?.pointee = skDup("the joint was already added to a physics world")
        return false
    }
    world.add(joint)
    return true
}

@_cdecl("sk_physics_world_remove_joint")
public func sk_physics_world_remove_joint(_ worldHandle: UnsafeMutableRawPointer?, _ jointHandle: UnsafeMutableRawPointer?) {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle), let joint: SKPhysicsJoint = skBorrow(jointHandle) else { return }
    world.remove(joint)
}

@_cdecl("sk_physics_world_remove_all_joints")
public func sk_physics_world_remove_all_joints(_ worldHandle: UnsafeMutableRawPointer?) {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return }
    world.removeAllJoints()
}
