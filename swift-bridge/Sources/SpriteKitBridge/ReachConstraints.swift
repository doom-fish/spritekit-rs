import SpriteKit

@_cdecl("sk_reach_constraints_new")
public func sk_reach_constraints_new(_ lowerAngleLimit: Double, _ upperAngleLimit: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKReachConstraints(lowerAngleLimit: lowerAngleLimit, upperAngleLimit: upperAngleLimit))
}

@_cdecl("sk_reach_constraints_get_lower_angle_limit")
public func sk_reach_constraints_get_lower_angle_limit(_ constraintsHandle: UnsafeMutableRawPointer?) -> Double {
    guard let constraints: SKReachConstraints = skBorrow(constraintsHandle) else { return 0 }
    return Double(constraints.lowerAngleLimit)
}

@_cdecl("sk_reach_constraints_set_lower_angle_limit")
public func sk_reach_constraints_set_lower_angle_limit(_ constraintsHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let constraints: SKReachConstraints = skBorrow(constraintsHandle) else { return }
    constraints.lowerAngleLimit = value
}

@_cdecl("sk_reach_constraints_get_upper_angle_limit")
public func sk_reach_constraints_get_upper_angle_limit(_ constraintsHandle: UnsafeMutableRawPointer?) -> Double {
    guard let constraints: SKReachConstraints = skBorrow(constraintsHandle) else { return 0 }
    return Double(constraints.upperAngleLimit)
}

@_cdecl("sk_reach_constraints_set_upper_angle_limit")
public func sk_reach_constraints_set_upper_angle_limit(_ constraintsHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let constraints: SKReachConstraints = skBorrow(constraintsHandle) else { return }
    constraints.upperAngleLimit = value
}
