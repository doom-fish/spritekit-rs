import SpriteKit
import Foundation

@_cdecl("sk_camera_node_new")
public func sk_camera_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKCameraNode())
}

@_cdecl("sk_camera_node_contains_node")
public func sk_camera_node_contains_node(_ cameraHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let camera: SKCameraNode = skBorrow(cameraHandle),
          let node: SKNode = skBorrow(nodeHandle)
    else { return false }
    return camera.contains(node)
}

@_cdecl("sk_camera_node_get_contained_node_count")
public func sk_camera_node_get_contained_node_count(_ cameraHandle: UnsafeMutableRawPointer?) -> Int {
    guard let camera: SKCameraNode = skBorrow(cameraHandle) else { return 0 }
    return camera.containedNodeSet().count
}

@_cdecl("sk_crop_node_new")
public func sk_crop_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKCropNode())
}

@_cdecl("sk_crop_node_get_mask_node")
public func sk_crop_node_get_mask_node(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKCropNode = skBorrow(nodeHandle),
          let maskNode = node.maskNode
    else { return nil }
    return skRetain(maskNode)
}

@_cdecl("sk_crop_node_set_mask_node")
public func sk_crop_node_set_mask_node(_ nodeHandle: UnsafeMutableRawPointer?, _ maskNodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKCropNode = skBorrow(nodeHandle) else { return }
    let maskNode: SKNode? = skBorrow(maskNodeHandle)
    node.maskNode = maskNode
}

@_cdecl("sk_field_node_drag")
public func sk_field_node_drag() -> UnsafeMutableRawPointer? {
    skRetain(SKFieldNode.dragField())
}

@_cdecl("sk_field_node_vortex")
public func sk_field_node_vortex() -> UnsafeMutableRawPointer? {
    skRetain(SKFieldNode.vortexField())
}

@_cdecl("sk_field_node_linear_gravity")
public func sk_field_node_linear_gravity(_ x: Float, _ y: Float, _ z: Float) -> UnsafeMutableRawPointer? {
    skRetain(SKFieldNode.linearGravityField(withVector: vector_float3(x, y, z)))
}

@_cdecl("sk_field_node_get_region")
public func sk_field_node_get_region(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKFieldNode = skBorrow(nodeHandle),
          let region = node.region
    else { return nil }
    return skRetain(region)
}

@_cdecl("sk_field_node_set_region")
public func sk_field_node_set_region(_ nodeHandle: UnsafeMutableRawPointer?, _ regionHandle: UnsafeMutableRawPointer?) {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return }
    let region: SKRegion? = skBorrow(regionHandle)
    node.region = region
}

@_cdecl("sk_field_node_get_strength")
public func sk_field_node_get_strength(_ nodeHandle: UnsafeMutableRawPointer?) -> Float {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return 0 }
    return node.strength
}

@_cdecl("sk_field_node_set_strength")
public func sk_field_node_set_strength(_ nodeHandle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return }
    node.strength = value
}

@_cdecl("sk_field_node_get_direction_x")
public func sk_field_node_get_direction_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Float {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return 0 }
    return node.direction.x
}

@_cdecl("sk_field_node_get_direction_y")
public func sk_field_node_get_direction_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Float {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return 0 }
    return node.direction.y
}

@_cdecl("sk_field_node_get_direction_z")
public func sk_field_node_get_direction_z(_ nodeHandle: UnsafeMutableRawPointer?) -> Float {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return 0 }
    return node.direction.z
}

@_cdecl("sk_field_node_set_direction")
public func sk_field_node_set_direction(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return }
    node.direction = vector_float3(x, y, z)
}

@_cdecl("sk_field_node_get_enabled")
public func sk_field_node_get_enabled(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return false }
    return node.isEnabled
}

@_cdecl("sk_field_node_set_enabled")
public func sk_field_node_set_enabled(_ nodeHandle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let node: SKFieldNode = skBorrow(nodeHandle) else { return }
    node.isEnabled = enabled
}

@_cdecl("sk_reference_node_new_with_file_named")
public func sk_reference_node_new_with_file_named(_ fileName: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let fileName else { return nil }
    return skRetain(SKReferenceNode(fileNamed: String(cString: fileName)))
}

@_cdecl("sk_reference_node_new_with_url_path")
public func sk_reference_node_new_with_url_path(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    return skRetain(SKReferenceNode(url: URL(fileURLWithPath: String(cString: path))))
}

@_cdecl("sk_reference_node_resolve")
public func sk_reference_node_resolve(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKReferenceNode = skBorrow(nodeHandle) else { return }
    node.resolve()
}

@_cdecl("sk_shape_node_new_with_rect")
public func sk_shape_node_new_with_rect(_ x: Double, _ y: Double, _ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKShapeNode(rect: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("sk_shape_node_new_with_rect_of_size")
public func sk_shape_node_new_with_rect_of_size(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKShapeNode(rectOf: CGSize(width: width, height: height)))
}

@_cdecl("sk_shape_node_new_with_circle")
public func sk_shape_node_new_with_circle(_ radius: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKShapeNode(circleOfRadius: radius))
}

@_cdecl("sk_shape_node_get_line_width")
public func sk_shape_node_get_line_width(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return 1 }
    return Double(node.lineWidth)
}

@_cdecl("sk_shape_node_set_line_width")
public func sk_shape_node_set_line_width(_ nodeHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return }
    node.lineWidth = value
}

@_cdecl("sk_shape_node_get_glow_width")
public func sk_shape_node_get_glow_width(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.glowWidth)
}

@_cdecl("sk_shape_node_set_glow_width")
public func sk_shape_node_set_glow_width(_ nodeHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return }
    node.glowWidth = value
}

@_cdecl("sk_shape_node_get_antialiased")
public func sk_shape_node_get_antialiased(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return true }
    return node.isAntialiased
}

@_cdecl("sk_shape_node_set_antialiased")
public func sk_shape_node_set_antialiased(_ nodeHandle: UnsafeMutableRawPointer?, _ antialiased: Bool) {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return }
    node.isAntialiased = antialiased
}

@_cdecl("sk_shape_node_get_line_length")
public func sk_shape_node_get_line_length(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.lineLength)
}

@_cdecl("sk_shape_node_set_stroke_color")
public func sk_shape_node_set_stroke_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return }
    node.strokeColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_shape_node_set_fill_color")
public func sk_shape_node_set_fill_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKShapeNode = skBorrow(nodeHandle) else { return }
    node.fillColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_transform_node_new")
public func sk_transform_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKTransformNode())
}

@_cdecl("sk_transform_node_get_x_rotation")
public func sk_transform_node_get_x_rotation(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKTransformNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.xRotation)
}

@_cdecl("sk_transform_node_set_x_rotation")
public func sk_transform_node_set_x_rotation(_ nodeHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let node: SKTransformNode = skBorrow(nodeHandle) else { return }
    node.xRotation = value
}

@_cdecl("sk_transform_node_get_y_rotation")
public func sk_transform_node_get_y_rotation(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKTransformNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.yRotation)
}

@_cdecl("sk_transform_node_set_y_rotation")
public func sk_transform_node_set_y_rotation(_ nodeHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let node: SKTransformNode = skBorrow(nodeHandle) else { return }
    node.yRotation = value
}
