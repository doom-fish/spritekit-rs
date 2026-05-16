import SpriteKit

@_cdecl("sk_effect_node_new")
public func sk_effect_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKEffectNode())
}

@_cdecl("sk_effect_node_get_should_enable_effects")
public func sk_effect_node_get_should_enable_effects(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKEffectNode = skBorrow(nodeHandle) else { return false }
    return node.shouldEnableEffects
}

@_cdecl("sk_effect_node_set_should_enable_effects")
public func sk_effect_node_set_should_enable_effects(_ nodeHandle: UnsafeMutableRawPointer?, _ enable: Bool) {
    guard let node: SKEffectNode = skBorrow(nodeHandle) else { return }
    node.shouldEnableEffects = enable
}

@_cdecl("sk_effect_node_get_should_rasterize")
public func sk_effect_node_get_should_rasterize(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKEffectNode = skBorrow(nodeHandle) else { return false }
    return node.shouldRasterize
}

@_cdecl("sk_effect_node_set_should_rasterize")
public func sk_effect_node_set_should_rasterize(_ nodeHandle: UnsafeMutableRawPointer?, _ rasterize: Bool) {
    guard let node: SKEffectNode = skBorrow(nodeHandle) else { return }
    node.shouldRasterize = rasterize
}

@_cdecl("sk_effect_node_get_blend_mode")
public func sk_effect_node_get_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let node: SKEffectNode = skBorrow(nodeHandle) else { return 0 }
    return Int32(node.blendMode.rawValue)
}

@_cdecl("sk_effect_node_set_blend_mode")
public func sk_effect_node_set_blend_mode(_ nodeHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let node: SKEffectNode = skBorrow(nodeHandle) else { return }
    node.blendMode = skBlendMode(mode)
}
