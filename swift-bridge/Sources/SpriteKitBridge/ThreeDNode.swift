import SceneKit
import SpriteKit

@_cdecl("sk_3d_node_new_with_viewport_size")
public func sk_3d_node_new_with_viewport_size(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SK3DNode(viewportSize: CGSize(width: width, height: height)))
}

@_cdecl("sk_3d_node_get_viewport_size_w")
public func sk_3d_node_get_viewport_size_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.viewportSize.width)
}

@_cdecl("sk_3d_node_get_viewport_size_h")
public func sk_3d_node_get_viewport_size_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.viewportSize.height)
}

@_cdecl("sk_3d_node_set_viewport_size")
public func sk_3d_node_set_viewport_size(_ nodeHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return }
    node.viewportSize = CGSize(width: width, height: height)
}

@_cdecl("sk_3d_node_set_empty_scene")
public func sk_3d_node_set_empty_scene(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return }
    node.scnScene = SCNScene()
}

@_cdecl("sk_3d_node_has_scene")
public func sk_3d_node_has_scene(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return false }
    return node.scnScene != nil
}

@_cdecl("sk_3d_node_get_scene_time")
public func sk_3d_node_get_scene_time(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return 0 }
    return node.sceneTime
}

@_cdecl("sk_3d_node_set_scene_time")
public func sk_3d_node_set_scene_time(_ nodeHandle: UnsafeMutableRawPointer?, _ time: Double) {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return }
    node.sceneTime = time
}

@_cdecl("sk_3d_node_get_playing")
public func sk_3d_node_get_playing(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return false }
    return node.isPlaying
}

@_cdecl("sk_3d_node_set_playing")
public func sk_3d_node_set_playing(_ nodeHandle: UnsafeMutableRawPointer?, _ playing: Bool) {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return }
    node.isPlaying = playing
}

@_cdecl("sk_3d_node_get_loops")
public func sk_3d_node_get_loops(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return false }
    return node.loops
}

@_cdecl("sk_3d_node_set_loops")
public func sk_3d_node_set_loops(_ nodeHandle: UnsafeMutableRawPointer?, _ loops: Bool) {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return }
    node.loops = loops
}

@_cdecl("sk_3d_node_get_autoenables_default_lighting")
public func sk_3d_node_get_autoenables_default_lighting(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return false }
    return node.autoenablesDefaultLighting
}

@_cdecl("sk_3d_node_set_autoenables_default_lighting")
public func sk_3d_node_set_autoenables_default_lighting(_ nodeHandle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let node: SK3DNode = skBorrow(nodeHandle) else { return }
    node.autoenablesDefaultLighting = enabled
}
