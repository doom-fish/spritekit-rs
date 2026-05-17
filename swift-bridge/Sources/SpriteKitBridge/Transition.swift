import SpriteKit

private func skTransitionDirection(_ rawValue: Int32) -> SKTransitionDirection {
    SKTransitionDirection(rawValue: Int(rawValue)) ?? .up
}

@_cdecl("sk_transition_cross_fade")
public func sk_transition_cross_fade(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.crossFade(withDuration: duration))
}

@_cdecl("sk_transition_fade")
public func sk_transition_fade(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.fade(withDuration: duration))
}

@_cdecl("sk_transition_fade_with_color")
public func sk_transition_fade_with_color(_ r: Float, _ g: Float, _ b: Float, _ a: Float, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.fade(with: skMakeColor(r: r, g: g, b: b, a: a), duration: duration))
}

@_cdecl("sk_transition_flip_horizontal")
public func sk_transition_flip_horizontal(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.flipHorizontal(withDuration: duration))
}

@_cdecl("sk_transition_flip_vertical")
public func sk_transition_flip_vertical(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.flipVertical(withDuration: duration))
}

@_cdecl("sk_transition_reveal")
public func sk_transition_reveal(_ direction: Int32, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.reveal(with: skTransitionDirection(direction), duration: duration))
}

@_cdecl("sk_transition_move_in")
public func sk_transition_move_in(_ direction: Int32, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.moveIn(with: skTransitionDirection(direction), duration: duration))
}

@_cdecl("sk_transition_push")
public func sk_transition_push(_ direction: Int32, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.push(with: skTransitionDirection(direction), duration: duration))
}

@_cdecl("sk_transition_doors_open_horizontal")
public func sk_transition_doors_open_horizontal(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.doorsOpenHorizontal(withDuration: duration))
}

@_cdecl("sk_transition_doors_open_vertical")
public func sk_transition_doors_open_vertical(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.doorsOpenVertical(withDuration: duration))
}

@_cdecl("sk_transition_doors_close_horizontal")
public func sk_transition_doors_close_horizontal(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.doorsCloseHorizontal(withDuration: duration))
}

@_cdecl("sk_transition_doors_close_vertical")
public func sk_transition_doors_close_vertical(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.doorsCloseVertical(withDuration: duration))
}

@_cdecl("sk_transition_doorway")
public func sk_transition_doorway(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKTransition.doorway(withDuration: duration))
}

@_cdecl("sk_transition_get_pauses_incoming_scene")
public func sk_transition_get_pauses_incoming_scene(_ transitionHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let transition: SKTransition = skBorrow(transitionHandle) else { return true }
    return transition.pausesIncomingScene
}

@_cdecl("sk_transition_set_pauses_incoming_scene")
public func sk_transition_set_pauses_incoming_scene(_ transitionHandle: UnsafeMutableRawPointer?, _ pauses: Bool) {
    guard let transition: SKTransition = skBorrow(transitionHandle) else { return }
    transition.pausesIncomingScene = pauses
}

@_cdecl("sk_transition_get_pauses_outgoing_scene")
public func sk_transition_get_pauses_outgoing_scene(_ transitionHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let transition: SKTransition = skBorrow(transitionHandle) else { return true }
    return transition.pausesOutgoingScene
}

@_cdecl("sk_transition_set_pauses_outgoing_scene")
public func sk_transition_set_pauses_outgoing_scene(_ transitionHandle: UnsafeMutableRawPointer?, _ pauses: Bool) {
    guard let transition: SKTransition = skBorrow(transitionHandle) else { return }
    transition.pausesOutgoingScene = pauses
}
