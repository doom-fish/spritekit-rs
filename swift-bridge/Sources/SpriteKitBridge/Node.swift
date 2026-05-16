import SpriteKit

@_cdecl("sk_node_new")
public func sk_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKNode())
}

@_cdecl("sk_node_add_child")
public func sk_node_add_child(_ parentHandle: UnsafeMutableRawPointer?, _ childHandle: UnsafeMutableRawPointer?) {
    guard let parent: SKNode = skBorrow(parentHandle),
          let child: SKNode = skBorrow(childHandle)
    else { return }
    parent.addChild(child)
}

@_cdecl("sk_node_move_to_parent")
public func sk_node_move_to_parent(_ nodeHandle: UnsafeMutableRawPointer?, _ parentHandle: UnsafeMutableRawPointer?) {
    guard let node: SKNode = skBorrow(nodeHandle),
          let parent: SKNode = skBorrow(parentHandle)
    else { return }
    node.move(toParent: parent)
}

@_cdecl("sk_node_remove_from_parent")
public func sk_node_remove_from_parent(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.removeFromParent()
}

@_cdecl("sk_node_remove_all_children")
public func sk_node_remove_all_children(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.removeAllChildren()
}

@_cdecl("sk_node_copy_name")
public func sk_node_copy_name(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let node: SKNode = skBorrow(nodeHandle) else { return nil }
    return skDup(node.name)
}

@_cdecl("sk_node_set_name")
public func sk_node_set_name(_ nodeHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.name = name.map(String.init(cString:))
}

@_cdecl("sk_node_get_frame_x")
public func sk_node_get_frame_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.frame.origin.x)
}

@_cdecl("sk_node_get_frame_y")
public func sk_node_get_frame_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.frame.origin.y)
}

@_cdecl("sk_node_get_frame_w")
public func sk_node_get_frame_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.frame.size.width)
}

@_cdecl("sk_node_get_frame_h")
public func sk_node_get_frame_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.frame.size.height)
}

@_cdecl("sk_node_calculate_accumulated_frame_x")
public func sk_node_calculate_accumulated_frame_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.calculateAccumulatedFrame().origin.x)
}

@_cdecl("sk_node_calculate_accumulated_frame_y")
public func sk_node_calculate_accumulated_frame_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.calculateAccumulatedFrame().origin.y)
}

@_cdecl("sk_node_calculate_accumulated_frame_w")
public func sk_node_calculate_accumulated_frame_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.calculateAccumulatedFrame().size.width)
}

@_cdecl("sk_node_calculate_accumulated_frame_h")
public func sk_node_calculate_accumulated_frame_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.calculateAccumulatedFrame().size.height)
}

@_cdecl("sk_node_get_position_x")
public func sk_node_get_position_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.position.x)
}

@_cdecl("sk_node_get_position_y")
public func sk_node_get_position_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.position.y)
}

@_cdecl("sk_node_set_position")
public func sk_node_set_position(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.position = CGPoint(x: x, y: y)
}

@_cdecl("sk_node_get_z_position")
public func sk_node_get_z_position(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.zPosition)
}

@_cdecl("sk_node_set_z_position")
public func sk_node_set_z_position(_ nodeHandle: UnsafeMutableRawPointer?, _ z: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.zPosition = CGFloat(z)
}

@_cdecl("sk_node_get_z_rotation")
public func sk_node_get_z_rotation(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.zRotation)
}

@_cdecl("sk_node_set_z_rotation")
public func sk_node_set_z_rotation(_ nodeHandle: UnsafeMutableRawPointer?, _ rotation: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.zRotation = CGFloat(rotation)
}

@_cdecl("sk_node_get_x_scale")
public func sk_node_get_x_scale(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 1 }
    return Double(node.xScale)
}

@_cdecl("sk_node_set_x_scale")
public func sk_node_set_x_scale(_ nodeHandle: UnsafeMutableRawPointer?, _ scale: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.xScale = CGFloat(scale)
}

@_cdecl("sk_node_get_y_scale")
public func sk_node_get_y_scale(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 1 }
    return Double(node.yScale)
}

@_cdecl("sk_node_set_y_scale")
public func sk_node_set_y_scale(_ nodeHandle: UnsafeMutableRawPointer?, _ scale: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.yScale = CGFloat(scale)
}

@_cdecl("sk_node_set_scale")
public func sk_node_set_scale(_ nodeHandle: UnsafeMutableRawPointer?, _ scale: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.setScale(CGFloat(scale))
}

@_cdecl("sk_node_get_alpha")
public func sk_node_get_alpha(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 1 }
    return Double(node.alpha)
}

@_cdecl("sk_node_set_alpha")
public func sk_node_set_alpha(_ nodeHandle: UnsafeMutableRawPointer?, _ alpha: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.alpha = CGFloat(alpha)
}

@_cdecl("sk_node_get_hidden")
public func sk_node_get_hidden(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKNode = skBorrow(nodeHandle) else { return false }
    return node.isHidden
}

@_cdecl("sk_node_set_hidden")
public func sk_node_set_hidden(_ nodeHandle: UnsafeMutableRawPointer?, _ hidden: Bool) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.isHidden = hidden
}

@_cdecl("sk_node_get_paused")
public func sk_node_get_paused(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKNode = skBorrow(nodeHandle) else { return false }
    return node.isPaused
}

@_cdecl("sk_node_set_paused")
public func sk_node_set_paused(_ nodeHandle: UnsafeMutableRawPointer?, _ paused: Bool) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.isPaused = paused
}

@_cdecl("sk_node_get_speed")
public func sk_node_get_speed(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 1 }
    return Double(node.speed)
}

@_cdecl("sk_node_set_speed")
public func sk_node_set_speed(_ nodeHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.speed = CGFloat(speed)
}

@_cdecl("sk_node_get_user_interaction_enabled")
public func sk_node_get_user_interaction_enabled(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKNode = skBorrow(nodeHandle) else { return false }
    return node.isUserInteractionEnabled
}

@_cdecl("sk_node_set_user_interaction_enabled")
public func sk_node_set_user_interaction_enabled(_ nodeHandle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.isUserInteractionEnabled = enabled
}

@_cdecl("sk_node_get_children_count")
public func sk_node_get_children_count(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return node.children.count
}

@_cdecl("sk_node_get_physics_body")
public func sk_node_get_physics_body(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKNode = skBorrow(nodeHandle), let body = node.physicsBody else { return nil }
    return skRetain(body)
}

@_cdecl("sk_node_set_physics_body")
public func sk_node_set_physics_body(_ nodeHandle: UnsafeMutableRawPointer?, _ bodyHandle: UnsafeMutableRawPointer?) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    let body: SKPhysicsBody? = skBorrow(bodyHandle)
    node.physicsBody = body
}

@_cdecl("sk_node_get_constraints_count")
public func sk_node_get_constraints_count(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SKNode = skBorrow(nodeHandle) else { return 0 }
    return node.constraints?.count ?? 0
}

@_cdecl("sk_node_set_constraints")
public func sk_node_set_constraints(_ nodeHandle: UnsafeMutableRawPointer?, _ rawConstraints: UnsafeMutableRawPointer?, _ count: Int) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.constraints = skConstraints(from: rawConstraints, count: count)
}

@_cdecl("sk_node_run_action")
public func sk_node_run_action(_ nodeHandle: UnsafeMutableRawPointer?, _ actionHandle: UnsafeMutableRawPointer?) {
    guard let node: SKNode = skBorrow(nodeHandle),
          let action: SKAction = skBorrow(actionHandle)
    else { return }
    node.run(action)
}

@_cdecl("sk_node_has_actions")
public func sk_node_has_actions(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKNode = skBorrow(nodeHandle) else { return false }
    return node.hasActions()
}

@_cdecl("sk_node_remove_all_actions")
public func sk_node_remove_all_actions(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKNode = skBorrow(nodeHandle) else { return }
    node.removeAllActions()
}

@_cdecl("sk_node_contains_point")
public func sk_node_contains_point(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Bool {
    guard let node: SKNode = skBorrow(nodeHandle) else { return false }
    return node.contains(CGPoint(x: x, y: y))
}
