import SpriteKit

@_cdecl("sk_range_new")
public func sk_range_new(_ lower: Double, _ upper: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKRange(lowerLimit: CGFloat(lower), upperLimit: CGFloat(upper)))
}

@_cdecl("sk_range_with_lower_limit")
public func sk_range_with_lower_limit(_ lower: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKRange(lowerLimit: CGFloat(lower)))
}

@_cdecl("sk_range_with_upper_limit")
public func sk_range_with_upper_limit(_ upper: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKRange(upperLimit: CGFloat(upper)))
}

@_cdecl("sk_range_with_constant")
public func sk_range_with_constant(_ value: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKRange(constantValue: CGFloat(value)))
}

@_cdecl("sk_range_with_variance")
public func sk_range_with_variance(_ value: Double, _ variance: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKRange(value: CGFloat(value), variance: CGFloat(variance)))
}

@_cdecl("sk_range_with_no_limits")
public func sk_range_with_no_limits() -> UnsafeMutableRawPointer? {
    skRetain(SKRange())
}

@_cdecl("sk_range_get_lower_limit")
public func sk_range_get_lower_limit(_ rangeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let range: SKRange = skBorrow(rangeHandle) else { return 0 }
    return Double(range.lowerLimit)
}

@_cdecl("sk_range_set_lower_limit")
public func sk_range_set_lower_limit(_ rangeHandle: UnsafeMutableRawPointer?, _ lower: Double) {
    guard let range: SKRange = skBorrow(rangeHandle) else { return }
    range.lowerLimit = CGFloat(lower)
}

@_cdecl("sk_range_get_upper_limit")
public func sk_range_get_upper_limit(_ rangeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let range: SKRange = skBorrow(rangeHandle) else { return 0 }
    return Double(range.upperLimit)
}

@_cdecl("sk_range_set_upper_limit")
public func sk_range_set_upper_limit(_ rangeHandle: UnsafeMutableRawPointer?, _ upper: Double) {
    guard let range: SKRange = skBorrow(rangeHandle) else { return }
    range.upperLimit = CGFloat(upper)
}

@_cdecl("sk_constraint_position_x")
public func sk_constraint_position_x(_ rangeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let range: SKRange = skBorrow(rangeHandle) else { return nil }
    return skRetain(SKConstraint.positionX(range))
}

@_cdecl("sk_constraint_position_y")
public func sk_constraint_position_y(_ rangeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let range: SKRange = skBorrow(rangeHandle) else { return nil }
    return skRetain(SKConstraint.positionY(range))
}

@_cdecl("sk_constraint_position_xy")
public func sk_constraint_position_xy(_ xRangeHandle: UnsafeMutableRawPointer?, _ yRangeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let xRange: SKRange = skBorrow(xRangeHandle),
          let yRange: SKRange = skBorrow(yRangeHandle)
    else { return nil }
    return skRetain(SKConstraint.positionX(xRange, y: yRange))
}

@_cdecl("sk_constraint_distance_to_node")
public func sk_constraint_distance_to_node(_ rangeHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let range: SKRange = skBorrow(rangeHandle),
          let node: SKNode = skBorrow(nodeHandle)
    else { return nil }
    return skRetain(SKConstraint.distance(range, to: node))
}

@_cdecl("sk_constraint_distance_to_point")
public func sk_constraint_distance_to_point(_ rangeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    guard let range: SKRange = skBorrow(rangeHandle) else { return nil }
    return skRetain(SKConstraint.distance(range, to: CGPoint(x: x, y: y)))
}

@_cdecl("sk_constraint_distance_to_point_in_node")
public func sk_constraint_distance_to_point_in_node(_ rangeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let range: SKRange = skBorrow(rangeHandle),
          let node: SKNode = skBorrow(nodeHandle)
    else { return nil }
    return skRetain(SKConstraint.distance(range, to: CGPoint(x: x, y: y), in: node))
}

@_cdecl("sk_constraint_z_rotation")
public func sk_constraint_z_rotation(_ rangeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let range: SKRange = skBorrow(rangeHandle) else { return nil }
    return skRetain(SKConstraint.zRotation(range))
}

@_cdecl("sk_constraint_orient_to_node")
public func sk_constraint_orient_to_node(_ nodeHandle: UnsafeMutableRawPointer?, _ offsetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKNode = skBorrow(nodeHandle),
          let offset: SKRange = skBorrow(offsetHandle)
    else { return nil }
    return skRetain(SKConstraint.orient(to: node, offset: offset))
}

@_cdecl("sk_constraint_orient_to_point")
public func sk_constraint_orient_to_point(_ x: Double, _ y: Double, _ offsetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let offset: SKRange = skBorrow(offsetHandle) else { return nil }
    return skRetain(SKConstraint.orient(to: CGPoint(x: x, y: y), offset: offset))
}

@_cdecl("sk_constraint_orient_to_point_in_node")
public func sk_constraint_orient_to_point_in_node(_ x: Double, _ y: Double, _ nodeHandle: UnsafeMutableRawPointer?, _ offsetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKNode = skBorrow(nodeHandle),
          let offset: SKRange = skBorrow(offsetHandle)
    else { return nil }
    return skRetain(SKConstraint.orient(to: CGPoint(x: x, y: y), in: node, offset: offset))
}

@_cdecl("sk_constraint_get_enabled")
public func sk_constraint_get_enabled(_ constraintHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let constraint: SKConstraint = skBorrow(constraintHandle) else { return false }
    return constraint.enabled
}

@_cdecl("sk_constraint_set_enabled")
public func sk_constraint_set_enabled(_ constraintHandle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let constraint: SKConstraint = skBorrow(constraintHandle) else { return }
    constraint.enabled = enabled
}

@_cdecl("sk_constraint_get_reference_node")
public func sk_constraint_get_reference_node(_ constraintHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let constraint: SKConstraint = skBorrow(constraintHandle), let referenceNode = constraint.referenceNode else { return nil }
    return skRetain(referenceNode)
}

@_cdecl("sk_constraint_set_reference_node")
public func sk_constraint_set_reference_node(_ constraintHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) {
    guard let constraint: SKConstraint = skBorrow(constraintHandle) else { return }
    let node: SKNode? = skBorrow(nodeHandle)
    constraint.referenceNode = node
}
