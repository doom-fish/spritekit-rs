import SpriteKit

@_cdecl("sk_sprite_node_new_with_texture")
public func sk_sprite_node_new_with_texture(_ textureHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let texture: SKTexture? = skBorrow(textureHandle)
    if let texture {
        return skRetain(SKSpriteNode(texture: texture))
    }
    return skRetain(SKSpriteNode())
}

@_cdecl("sk_sprite_node_new_with_color")
public func sk_sprite_node_new_with_color(
    _ r: Float, _ g: Float, _ b: Float, _ a: Float,
    _ width: Double, _ height: Double
) -> UnsafeMutableRawPointer? {
    let color = skMakeColor(r: r, g: g, b: b, a: a)
    return skRetain(SKSpriteNode(color: color, size: CGSize(width: width, height: height)))
}

@_cdecl("sk_sprite_node_new_image_named")
public func sk_sprite_node_new_image_named(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    return skRetain(SKSpriteNode(imageNamed: String(cString: name)))
}

@_cdecl("sk_sprite_node_get_texture")
public func sk_sprite_node_get_texture(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKSpriteNode = skBorrow(nodeHandle), let texture = node.texture else { return nil }
    return skRetain(texture)
}

@_cdecl("sk_sprite_node_set_texture")
public func sk_sprite_node_set_texture(_ nodeHandle: UnsafeMutableRawPointer?, _ textureHandle: UnsafeMutableRawPointer?) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    let texture: SKTexture? = skBorrow(textureHandle)
    node.texture = texture
}

@_cdecl("sk_sprite_node_get_normal_texture")
public func sk_sprite_node_get_normal_texture(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKSpriteNode = skBorrow(nodeHandle), let texture = node.normalTexture else { return nil }
    return skRetain(texture)
}

@_cdecl("sk_sprite_node_set_normal_texture")
public func sk_sprite_node_set_normal_texture(_ nodeHandle: UnsafeMutableRawPointer?, _ textureHandle: UnsafeMutableRawPointer?) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    let texture: SKTexture? = skBorrow(textureHandle)
    node.normalTexture = texture
}

@_cdecl("sk_sprite_node_get_size_w")
public func sk_sprite_node_get_size_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.size.width)
}

@_cdecl("sk_sprite_node_get_size_h")
public func sk_sprite_node_get_size_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.size.height)
}

@_cdecl("sk_sprite_node_set_size")
public func sk_sprite_node_set_size(_ nodeHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.size = CGSize(width: width, height: height)
}

@_cdecl("sk_sprite_node_scale_to_size")
public func sk_sprite_node_scale_to_size(_ nodeHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.scale(to: CGSize(width: width, height: height))
}

@_cdecl("sk_sprite_node_get_anchor_x")
public func sk_sprite_node_get_anchor_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0.5 }
    return Double(node.anchorPoint.x)
}

@_cdecl("sk_sprite_node_get_anchor_y")
public func sk_sprite_node_get_anchor_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0.5 }
    return Double(node.anchorPoint.y)
}

@_cdecl("sk_sprite_node_set_anchor_point")
public func sk_sprite_node_set_anchor_point(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.anchorPoint = CGPoint(x: x, y: y)
}

@_cdecl("sk_sprite_node_set_color")
public func sk_sprite_node_set_color(
    _ nodeHandle: UnsafeMutableRawPointer?,
    _ r: Float, _ g: Float, _ b: Float, _ a: Float
) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.color = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_sprite_node_get_color_blend_factor")
public func sk_sprite_node_get_color_blend_factor(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.colorBlendFactor)
}

@_cdecl("sk_sprite_node_set_color_blend_factor")
public func sk_sprite_node_set_color_blend_factor(_ nodeHandle: UnsafeMutableRawPointer?, _ factor: Double) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.colorBlendFactor = CGFloat(factor)
}

@_cdecl("sk_sprite_node_get_blend_mode")
public func sk_sprite_node_get_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Int32(node.blendMode.rawValue)
}

@_cdecl("sk_sprite_node_set_blend_mode")
public func sk_sprite_node_set_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.blendMode = skBlendMode(mode)
}

@_cdecl("sk_sprite_node_get_lighting_bitmask")
public func sk_sprite_node_get_lighting_bitmask(_ nodeHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return node.lightingBitMask
}

@_cdecl("sk_sprite_node_set_lighting_bitmask")
public func sk_sprite_node_set_lighting_bitmask(_ nodeHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.lightingBitMask = mask
}

@_cdecl("sk_sprite_node_get_shadow_cast_bitmask")
public func sk_sprite_node_get_shadow_cast_bitmask(_ nodeHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return node.shadowCastBitMask
}

@_cdecl("sk_sprite_node_set_shadow_cast_bitmask")
public func sk_sprite_node_set_shadow_cast_bitmask(_ nodeHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.shadowCastBitMask = mask
}

@_cdecl("sk_sprite_node_get_shadowed_bitmask")
public func sk_sprite_node_get_shadowed_bitmask(_ nodeHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return node.shadowedBitMask
}

@_cdecl("sk_sprite_node_set_shadowed_bitmask")
public func sk_sprite_node_set_shadowed_bitmask(_ nodeHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.shadowedBitMask = mask
}

@_cdecl("sk_sprite_node_get_center_rect_x")
public func sk_sprite_node_get_center_rect_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.centerRect.origin.x)
}

@_cdecl("sk_sprite_node_get_center_rect_y")
public func sk_sprite_node_get_center_rect_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.centerRect.origin.y)
}

@_cdecl("sk_sprite_node_get_center_rect_w")
public func sk_sprite_node_get_center_rect_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.centerRect.size.width)
}

@_cdecl("sk_sprite_node_get_center_rect_h")
public func sk_sprite_node_get_center_rect_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.centerRect.size.height)
}

@_cdecl("sk_sprite_node_set_center_rect")
public func sk_sprite_node_set_center_rect(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    node.centerRect = CGRect(x: x, y: y, width: width, height: height)
}

@_cdecl("sk_sprite_node_get_shader")
public func sk_sprite_node_get_shader(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SKSpriteNode = skBorrow(nodeHandle), let shader = node.shader else { return nil }
    return skRetain(shader)
}

@_cdecl("sk_sprite_node_set_shader")
public func sk_sprite_node_set_shader(_ nodeHandle: UnsafeMutableRawPointer?, _ shaderHandle: UnsafeMutableRawPointer?) {
    guard let node: SKSpriteNode = skBorrow(nodeHandle) else { return }
    let shader: SKShader? = skBorrow(shaderHandle)
    node.shader = shader
}
