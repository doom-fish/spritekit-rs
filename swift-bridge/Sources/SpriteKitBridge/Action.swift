import SpriteKit

@_cdecl("sk_action_move_by")
public func sk_action_move_by(_ dx: Double, _ dy: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.moveBy(x: CGFloat(dx), y: CGFloat(dy), duration: duration))
}

@_cdecl("sk_action_move_to")
public func sk_action_move_to(_ x: Double, _ y: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.move(to: CGPoint(x: x, y: y), duration: duration))
}

@_cdecl("sk_action_rotate_by")
public func sk_action_rotate_by(_ angle: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.rotate(byAngle: CGFloat(angle), duration: duration))
}

@_cdecl("sk_action_rotate_to")
public func sk_action_rotate_to(_ angle: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.rotate(toAngle: CGFloat(angle), duration: duration))
}

@_cdecl("sk_action_scale_by")
public func sk_action_scale_by(_ scale: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.scale(by: CGFloat(scale), duration: duration))
}

@_cdecl("sk_action_scale_to")
public func sk_action_scale_to(_ scale: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.scale(to: CGFloat(scale), duration: duration))
}

@_cdecl("sk_action_resize_to")
public func sk_action_resize_to(_ width: Double, _ height: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.resize(toWidth: CGFloat(width), height: CGFloat(height), duration: duration))
}

@_cdecl("sk_action_fade_in")
public func sk_action_fade_in(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.fadeIn(withDuration: duration))
}

@_cdecl("sk_action_fade_out")
public func sk_action_fade_out(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.fadeOut(withDuration: duration))
}

@_cdecl("sk_action_fade_to")
public func sk_action_fade_to(_ alpha: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.fadeAlpha(to: CGFloat(alpha), duration: duration))
}

@_cdecl("sk_action_fade_alpha_by")
public func sk_action_fade_alpha_by(_ alpha: Double, _ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.fadeAlpha(by: CGFloat(alpha), duration: duration))
}

@_cdecl("sk_action_hide")
public func sk_action_hide() -> UnsafeMutableRawPointer? {
    skRetain(SKAction.hide())
}

@_cdecl("sk_action_unhide")
public func sk_action_unhide() -> UnsafeMutableRawPointer? {
    skRetain(SKAction.unhide())
}

@_cdecl("sk_action_set_texture")
public func sk_action_set_texture(_ textureHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return nil }
    return skRetain(SKAction.setTexture(texture))
}

@_cdecl("sk_action_animate_with_textures")
public func sk_action_animate_with_textures(
    _ rawTextures: UnsafeMutableRawPointer?,
    _ count: Int,
    _ timePerFrame: Double
) -> UnsafeMutableRawPointer? {
    let textures = skTextures(from: rawTextures, count: count)
    guard !textures.isEmpty else { return nil }
    return skRetain(SKAction.animate(with: textures, timePerFrame: timePerFrame))
}

@_cdecl("sk_action_wait")
public func sk_action_wait(_ duration: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.wait(forDuration: duration))
}

@_cdecl("sk_action_sequence")
public func sk_action_sequence(_ rawActions: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.sequence(skActions(from: rawActions, count: count)))
}

@_cdecl("sk_action_group")
public func sk_action_group(_ rawActions: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    skRetain(SKAction.group(skActions(from: rawActions, count: count)))
}

@_cdecl("sk_action_repeat")
public func sk_action_repeat(_ actionHandle: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let action: SKAction = skBorrow(actionHandle) else { return nil }
    return skRetain(SKAction.repeat(action, count: count))
}

@_cdecl("sk_action_repeat_forever")
public func sk_action_repeat_forever(_ actionHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action: SKAction = skBorrow(actionHandle) else { return nil }
    return skRetain(SKAction.repeatForever(action))
}

@_cdecl("sk_action_get_duration")
public func sk_action_get_duration(_ actionHandle: UnsafeMutableRawPointer?) -> Double {
    guard let action: SKAction = skBorrow(actionHandle) else { return 0 }
    return action.duration
}

@_cdecl("sk_action_set_duration")
public func sk_action_set_duration(_ actionHandle: UnsafeMutableRawPointer?, _ duration: Double) {
    guard let action: SKAction = skBorrow(actionHandle) else { return }
    action.duration = duration
}

@_cdecl("sk_action_get_timing_mode")
public func sk_action_get_timing_mode(_ actionHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let action: SKAction = skBorrow(actionHandle) else { return 0 }
    return Int32(action.timingMode.rawValue)
}

@_cdecl("sk_action_set_timing_mode")
public func sk_action_set_timing_mode(_ actionHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let action: SKAction = skBorrow(actionHandle) else { return }
    action.timingMode = SKActionTimingMode(rawValue: Int(mode)) ?? .linear
}

@_cdecl("sk_action_get_speed")
public func sk_action_get_speed(_ actionHandle: UnsafeMutableRawPointer?) -> Double {
    guard let action: SKAction = skBorrow(actionHandle) else { return 1 }
    return Double(action.speed)
}

@_cdecl("sk_action_set_speed")
public func sk_action_set_speed(_ actionHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let action: SKAction = skBorrow(actionHandle) else { return }
    action.speed = CGFloat(speed)
}

@_cdecl("sk_action_reversed")
public func sk_action_reversed(_ actionHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action: SKAction = skBorrow(actionHandle) else { return nil }
    return skRetain(action.reversed())
}
