import AVFoundation
import SpriteKit

@_cdecl("sk_video_node_new")
public func sk_video_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKVideoNode(avPlayer: AVPlayer()))
}

@_cdecl("sk_video_node_play")
public func sk_video_node_play(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return }
    node.play()
}

@_cdecl("sk_video_node_pause")
public func sk_video_node_pause(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return }
    node.pause()
}

@_cdecl("sk_video_node_get_size_w")
public func sk_video_node_get_size_w(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.size.width)
}

@_cdecl("sk_video_node_get_size_h")
public func sk_video_node_get_size_h(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return 0 }
    return Double(node.size.height)
}

@_cdecl("sk_video_node_set_size")
public func sk_video_node_set_size(_ nodeHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return }
    node.size = CGSize(width: width, height: height)
}

@_cdecl("sk_video_node_get_anchor_x")
public func sk_video_node_get_anchor_x(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return 0.5 }
    return Double(node.anchorPoint.x)
}

@_cdecl("sk_video_node_get_anchor_y")
public func sk_video_node_get_anchor_y(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return 0.5 }
    return Double(node.anchorPoint.y)
}

@_cdecl("sk_video_node_set_anchor_point")
public func sk_video_node_set_anchor_point(_ nodeHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let node: SKVideoNode = skBorrow(nodeHandle) else { return }
    node.anchorPoint = CGPoint(x: x, y: y)
}
