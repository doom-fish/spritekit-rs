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
