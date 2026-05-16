import AVFoundation
import SpriteKit

@_cdecl("sk_audio_node_new")
public func sk_audio_node_new() -> UnsafeMutableRawPointer? {
    skRetain(SKAudioNode(avAudioNode: AVAudioPlayerNode()))
}

@_cdecl("sk_audio_node_get_autoplay_looped")
public func sk_audio_node_get_autoplay_looped(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKAudioNode = skBorrow(nodeHandle) else { return true }
    return node.autoplayLooped
}

@_cdecl("sk_audio_node_set_autoplay_looped")
public func sk_audio_node_set_autoplay_looped(_ nodeHandle: UnsafeMutableRawPointer?, _ autoplay: Bool) {
    guard let node: SKAudioNode = skBorrow(nodeHandle) else { return }
    node.autoplayLooped = autoplay
}

@_cdecl("sk_audio_node_get_positional")
public func sk_audio_node_get_positional(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKAudioNode = skBorrow(nodeHandle) else { return false }
    return node.isPositional
}

@_cdecl("sk_audio_node_set_positional")
public func sk_audio_node_set_positional(_ nodeHandle: UnsafeMutableRawPointer?, _ positional: Bool) {
    guard let node: SKAudioNode = skBorrow(nodeHandle) else { return }
    node.isPositional = positional
}

@_cdecl("sk_audio_node_has_av_audio_node")
public func sk_audio_node_has_av_audio_node(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SKAudioNode = skBorrow(nodeHandle) else { return false }
    return node.avAudioNode != nil
}
