import SpriteKit

@_cdecl("sk_light_node_new")
public func sk_light_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKLightNode())
}

@_cdecl("sk_light_node_get_enabled")
public func sk_light_node_get_enabled(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return false }
    return node.isEnabled
}

@_cdecl("sk_light_node_set_enabled")
public func sk_light_node_set_enabled(_ nodeHandle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return }
    node.isEnabled = enabled
}

@_cdecl("sk_light_node_set_light_color")
public func sk_light_node_set_light_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return }
    node.lightColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_light_node_set_ambient_color")
public func sk_light_node_set_ambient_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return }
    node.ambientColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_light_node_set_shadow_color")
public func sk_light_node_set_shadow_color(_ nodeHandle: UnsafeMutableRawPointer?, _ r: Float, _ g: Float, _ b: Float, _ a: Float) {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return }
    node.shadowColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_light_node_get_falloff")
public func sk_light_node_get_falloff(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.falloff)
}

@_cdecl("sk_light_node_set_falloff")
public func sk_light_node_set_falloff(_ nodeHandle: UnsafeMutableRawPointer?, _ falloff: Double) {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return }
    node.falloff = CGFloat(falloff)
}

@_cdecl("sk_light_node_get_category_bitmask")
public func sk_light_node_get_category_bitmask(_ nodeHandle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return 0 }
    return node.categoryBitMask
}

@_cdecl("sk_light_node_set_category_bitmask")
public func sk_light_node_set_category_bitmask(_ nodeHandle: UnsafeMutableRawPointer?, _ mask: UInt32) {
    guard let node: SKLightNode = skBorrow(nodeHandle) else { return }
    node.categoryBitMask = mask
}
