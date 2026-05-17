import SpriteKit

@_cdecl("sk_physics_joint_get_reaction_force_dx")
public func sk_physics_joint_get_reaction_force_dx(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJoint = skBorrow(jointHandle) else { return 0 }
    return Double(joint.reactionForce.dx)
}

@_cdecl("sk_physics_joint_get_reaction_force_dy")
public func sk_physics_joint_get_reaction_force_dy(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJoint = skBorrow(jointHandle) else { return 0 }
    return Double(joint.reactionForce.dy)
}

@_cdecl("sk_physics_joint_get_reaction_torque")
public func sk_physics_joint_get_reaction_torque(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJoint = skBorrow(jointHandle) else { return 0 }
    return Double(joint.reactionTorque)
}

@_cdecl("sk_physics_joint_pin_new")
public func sk_physics_joint_pin_new(_ bodyAHandle: UnsafeMutableRawPointer?, _ bodyBHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    guard let bodyA: SKPhysicsBody = skBorrow(bodyAHandle),
          let bodyB: SKPhysicsBody = skBorrow(bodyBHandle)
    else { return nil }
    return skRetain(SKPhysicsJointPin.joint(withBodyA: bodyA, bodyB: bodyB, anchor: CGPoint(x: x, y: y)))
}

@_cdecl("sk_physics_joint_pin_get_should_enable_limits")
public func sk_physics_joint_pin_get_should_enable_limits(_ jointHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return false }
    return joint.shouldEnableLimits
}

@_cdecl("sk_physics_joint_pin_set_should_enable_limits")
public func sk_physics_joint_pin_set_should_enable_limits(_ jointHandle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return }
    joint.shouldEnableLimits = enabled
}

@_cdecl("sk_physics_joint_pin_get_lower_angle_limit")
public func sk_physics_joint_pin_get_lower_angle_limit(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return 0 }
    return Double(joint.lowerAngleLimit)
}

@_cdecl("sk_physics_joint_pin_set_lower_angle_limit")
public func sk_physics_joint_pin_set_lower_angle_limit(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return }
    joint.lowerAngleLimit = CGFloat(value)
}

@_cdecl("sk_physics_joint_pin_get_upper_angle_limit")
public func sk_physics_joint_pin_get_upper_angle_limit(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return 0 }
    return Double(joint.upperAngleLimit)
}

@_cdecl("sk_physics_joint_pin_set_upper_angle_limit")
public func sk_physics_joint_pin_set_upper_angle_limit(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return }
    joint.upperAngleLimit = CGFloat(value)
}

@_cdecl("sk_physics_joint_pin_get_friction_torque")
public func sk_physics_joint_pin_get_friction_torque(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return 0 }
    return Double(joint.frictionTorque)
}

@_cdecl("sk_physics_joint_pin_set_friction_torque")
public func sk_physics_joint_pin_set_friction_torque(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return }
    joint.frictionTorque = CGFloat(value)
}

@_cdecl("sk_physics_joint_pin_get_rotation_speed")
public func sk_physics_joint_pin_get_rotation_speed(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return 0 }
    return Double(joint.rotationSpeed)
}

@_cdecl("sk_physics_joint_pin_set_rotation_speed")
public func sk_physics_joint_pin_set_rotation_speed(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointPin = skBorrow(jointHandle) else { return }
    joint.rotationSpeed = CGFloat(value)
}

@_cdecl("sk_physics_joint_spring_new")
public func sk_physics_joint_spring_new(_ bodyAHandle: UnsafeMutableRawPointer?, _ bodyBHandle: UnsafeMutableRawPointer?, _ anchorAX: Double, _ anchorAY: Double, _ anchorBX: Double, _ anchorBY: Double) -> UnsafeMutableRawPointer? {
    guard let bodyA: SKPhysicsBody = skBorrow(bodyAHandle),
          let bodyB: SKPhysicsBody = skBorrow(bodyBHandle)
    else { return nil }
    return skRetain(SKPhysicsJointSpring.joint(withBodyA: bodyA, bodyB: bodyB, anchorA: CGPoint(x: anchorAX, y: anchorAY), anchorB: CGPoint(x: anchorBX, y: anchorBY)))
}

@_cdecl("sk_physics_joint_spring_get_damping")
public func sk_physics_joint_spring_get_damping(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointSpring = skBorrow(jointHandle) else { return 0 }
    return Double(joint.damping)
}

@_cdecl("sk_physics_joint_spring_set_damping")
public func sk_physics_joint_spring_set_damping(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointSpring = skBorrow(jointHandle) else { return }
    joint.damping = CGFloat(value)
}

@_cdecl("sk_physics_joint_spring_get_frequency")
public func sk_physics_joint_spring_get_frequency(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointSpring = skBorrow(jointHandle) else { return 0 }
    return Double(joint.frequency)
}

@_cdecl("sk_physics_joint_spring_set_frequency")
public func sk_physics_joint_spring_set_frequency(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointSpring = skBorrow(jointHandle) else { return }
    joint.frequency = CGFloat(value)
}

@_cdecl("sk_physics_joint_fixed_new")
public func sk_physics_joint_fixed_new(_ bodyAHandle: UnsafeMutableRawPointer?, _ bodyBHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    guard let bodyA: SKPhysicsBody = skBorrow(bodyAHandle),
          let bodyB: SKPhysicsBody = skBorrow(bodyBHandle)
    else { return nil }
    return skRetain(SKPhysicsJointFixed.joint(withBodyA: bodyA, bodyB: bodyB, anchor: CGPoint(x: x, y: y)))
}

@_cdecl("sk_physics_joint_sliding_new")
public func sk_physics_joint_sliding_new(_ bodyAHandle: UnsafeMutableRawPointer?, _ bodyBHandle: UnsafeMutableRawPointer?, _ anchorX: Double, _ anchorY: Double, _ axisDX: Double, _ axisDY: Double) -> UnsafeMutableRawPointer? {
    guard let bodyA: SKPhysicsBody = skBorrow(bodyAHandle),
          let bodyB: SKPhysicsBody = skBorrow(bodyBHandle)
    else { return nil }
    return skRetain(SKPhysicsJointSliding.joint(withBodyA: bodyA, bodyB: bodyB, anchor: CGPoint(x: anchorX, y: anchorY), axis: CGVector(dx: axisDX, dy: axisDY)))
}

@_cdecl("sk_physics_joint_sliding_get_should_enable_limits")
public func sk_physics_joint_sliding_get_should_enable_limits(_ jointHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let joint: SKPhysicsJointSliding = skBorrow(jointHandle) else { return false }
    return joint.shouldEnableLimits
}

@_cdecl("sk_physics_joint_sliding_set_should_enable_limits")
public func sk_physics_joint_sliding_set_should_enable_limits(_ jointHandle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let joint: SKPhysicsJointSliding = skBorrow(jointHandle) else { return }
    joint.shouldEnableLimits = enabled
}

@_cdecl("sk_physics_joint_sliding_get_lower_distance_limit")
public func sk_physics_joint_sliding_get_lower_distance_limit(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointSliding = skBorrow(jointHandle) else { return 0 }
    return Double(joint.lowerDistanceLimit)
}

@_cdecl("sk_physics_joint_sliding_set_lower_distance_limit")
public func sk_physics_joint_sliding_set_lower_distance_limit(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointSliding = skBorrow(jointHandle) else { return }
    joint.lowerDistanceLimit = CGFloat(value)
}

@_cdecl("sk_physics_joint_sliding_get_upper_distance_limit")
public func sk_physics_joint_sliding_get_upper_distance_limit(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointSliding = skBorrow(jointHandle) else { return 0 }
    return Double(joint.upperDistanceLimit)
}

@_cdecl("sk_physics_joint_sliding_set_upper_distance_limit")
public func sk_physics_joint_sliding_set_upper_distance_limit(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointSliding = skBorrow(jointHandle) else { return }
    joint.upperDistanceLimit = CGFloat(value)
}

@_cdecl("sk_physics_joint_limit_new")
public func sk_physics_joint_limit_new(_ bodyAHandle: UnsafeMutableRawPointer?, _ bodyBHandle: UnsafeMutableRawPointer?, _ anchorAX: Double, _ anchorAY: Double, _ anchorBX: Double, _ anchorBY: Double) -> UnsafeMutableRawPointer? {
    guard let bodyA: SKPhysicsBody = skBorrow(bodyAHandle),
          let bodyB: SKPhysicsBody = skBorrow(bodyBHandle)
    else { return nil }
    return skRetain(SKPhysicsJointLimit.joint(withBodyA: bodyA, bodyB: bodyB, anchorA: CGPoint(x: anchorAX, y: anchorAY), anchorB: CGPoint(x: anchorBX, y: anchorBY)))
}

@_cdecl("sk_physics_joint_limit_get_max_length")
public func sk_physics_joint_limit_get_max_length(_ jointHandle: UnsafeMutableRawPointer?) -> Double {
    guard let joint: SKPhysicsJointLimit = skBorrow(jointHandle) else { return 0 }
    return Double(joint.maxLength)
}

@_cdecl("sk_physics_joint_limit_set_max_length")
public func sk_physics_joint_limit_set_max_length(_ jointHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let joint: SKPhysicsJointLimit = skBorrow(jointHandle) else { return }
    joint.maxLength = CGFloat(value)
}
