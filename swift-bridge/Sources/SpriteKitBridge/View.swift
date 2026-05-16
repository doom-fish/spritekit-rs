import AppKit
import SpriteKit

@_cdecl("sk_view_new_with_frame")
public func sk_view_new_with_frame(_ x: Double, _ y: Double, _ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKView(frame: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("sk_view_get_paused")
public func sk_view_get_paused(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.isPaused
}

@_cdecl("sk_view_set_paused")
public func sk_view_set_paused(_ viewHandle: UnsafeMutableRawPointer?, _ paused: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.isPaused = paused
}

@_cdecl("sk_view_get_shows_fps")
public func sk_view_get_shows_fps(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.showsFPS
}

@_cdecl("sk_view_set_shows_fps")
public func sk_view_set_shows_fps(_ viewHandle: UnsafeMutableRawPointer?, _ shows: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.showsFPS = shows
}

@_cdecl("sk_view_get_shows_draw_count")
public func sk_view_get_shows_draw_count(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.showsDrawCount
}

@_cdecl("sk_view_set_shows_draw_count")
public func sk_view_set_shows_draw_count(_ viewHandle: UnsafeMutableRawPointer?, _ shows: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.showsDrawCount = shows
}

@_cdecl("sk_view_get_shows_node_count")
public func sk_view_get_shows_node_count(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.showsNodeCount
}

@_cdecl("sk_view_set_shows_node_count")
public func sk_view_set_shows_node_count(_ viewHandle: UnsafeMutableRawPointer?, _ shows: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.showsNodeCount = shows
}

@_cdecl("sk_view_get_shows_quad_count")
public func sk_view_get_shows_quad_count(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.showsQuadCount
}

@_cdecl("sk_view_set_shows_quad_count")
public func sk_view_set_shows_quad_count(_ viewHandle: UnsafeMutableRawPointer?, _ shows: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.showsQuadCount = shows
}

@_cdecl("sk_view_get_shows_physics")
public func sk_view_get_shows_physics(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.showsPhysics
}

@_cdecl("sk_view_set_shows_physics")
public func sk_view_set_shows_physics(_ viewHandle: UnsafeMutableRawPointer?, _ shows: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.showsPhysics = shows
}

@_cdecl("sk_view_get_shows_fields")
public func sk_view_get_shows_fields(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.showsFields
}

@_cdecl("sk_view_set_shows_fields")
public func sk_view_set_shows_fields(_ viewHandle: UnsafeMutableRawPointer?, _ shows: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.showsFields = shows
}

@_cdecl("sk_view_get_asynchronous")
public func sk_view_get_asynchronous(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return true }
    return view.isAsynchronous
}

@_cdecl("sk_view_set_asynchronous")
public func sk_view_set_asynchronous(_ viewHandle: UnsafeMutableRawPointer?, _ asynchronous: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.isAsynchronous = asynchronous
}

@_cdecl("sk_view_get_allows_transparency")
public func sk_view_get_allows_transparency(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.allowsTransparency
}

@_cdecl("sk_view_set_allows_transparency")
public func sk_view_set_allows_transparency(_ viewHandle: UnsafeMutableRawPointer?, _ allows: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.allowsTransparency = allows
}

@_cdecl("sk_view_get_ignores_sibling_order")
public func sk_view_get_ignores_sibling_order(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.ignoresSiblingOrder
}

@_cdecl("sk_view_set_ignores_sibling_order")
public func sk_view_set_ignores_sibling_order(_ viewHandle: UnsafeMutableRawPointer?, _ ignores: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.ignoresSiblingOrder = ignores
}

@_cdecl("sk_view_get_should_cull_non_visible_nodes")
public func sk_view_get_should_cull_non_visible_nodes(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.shouldCullNonVisibleNodes
}

@_cdecl("sk_view_set_should_cull_non_visible_nodes")
public func sk_view_set_should_cull_non_visible_nodes(_ viewHandle: UnsafeMutableRawPointer?, _ shouldCull: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.shouldCullNonVisibleNodes = shouldCull
}

@_cdecl("sk_view_get_preferred_frames_per_second")
public func sk_view_get_preferred_frames_per_second(_ viewHandle: UnsafeMutableRawPointer?) -> Int {
    guard let view: SKView = skBorrow(viewHandle) else { return 0 }
    return view.preferredFramesPerSecond
}

@_cdecl("sk_view_set_preferred_frames_per_second")
public func sk_view_set_preferred_frames_per_second(_ viewHandle: UnsafeMutableRawPointer?, _ fps: Int) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.preferredFramesPerSecond = fps
}

@_cdecl("sk_view_get_disable_depth_stencil_buffer")
public func sk_view_get_disable_depth_stencil_buffer(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SKView = skBorrow(viewHandle) else { return false }
    return view.disableDepthStencilBuffer
}

@_cdecl("sk_view_set_disable_depth_stencil_buffer")
public func sk_view_set_disable_depth_stencil_buffer(_ viewHandle: UnsafeMutableRawPointer?, _ disable: Bool) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    view.disableDepthStencilBuffer = disable
}

@_cdecl("sk_view_present_scene")
public func sk_view_present_scene(_ viewHandle: UnsafeMutableRawPointer?, _ sceneHandle: UnsafeMutableRawPointer?) {
    guard let view: SKView = skBorrow(viewHandle) else { return }
    let scene: SKScene? = skBorrow(sceneHandle)
    view.presentScene(scene)
}

@_cdecl("sk_view_get_scene")
public func sk_view_get_scene(_ viewHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: SKView = skBorrow(viewHandle), let scene = view.scene else { return nil }
    return skRetain(scene)
}

@_cdecl("sk_view_texture_from_node")
public func sk_view_texture_from_node(_ viewHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: SKView = skBorrow(viewHandle),
          let node: SKNode = skBorrow(nodeHandle),
          let texture = view.texture(from: node)
    else { return nil }
    return skRetain(texture)
}
