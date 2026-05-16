import Foundation
import SpriteKit

@_cdecl("sk_keyframe_sequence_new_with_capacity")
public func sk_keyframe_sequence_new_with_capacity(_ capacity: Int) -> UnsafeMutableRawPointer? {
    skRetain(SKKeyframeSequence(capacity: capacity))
}

@_cdecl("sk_keyframe_sequence_count")
public func sk_keyframe_sequence_count(_ sequenceHandle: UnsafeMutableRawPointer?) -> Int {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return 0 }
    return sequence.count()
}

@_cdecl("sk_keyframe_sequence_add_scalar")
public func sk_keyframe_sequence_add_scalar(_ sequenceHandle: UnsafeMutableRawPointer?, _ value: Double, _ time: Double) {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return }
    sequence.addKeyframeValue(NSNumber(value: value), time: CGFloat(time))
}

@_cdecl("sk_keyframe_sequence_remove_last")
public func sk_keyframe_sequence_remove_last(_ sequenceHandle: UnsafeMutableRawPointer?) {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return }
    sequence.removeLastKeyframe()
}

@_cdecl("sk_keyframe_sequence_remove_at")
public func sk_keyframe_sequence_remove_at(_ sequenceHandle: UnsafeMutableRawPointer?, _ index: Int) {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return }
    sequence.removeKeyframe(at: index)
}

@_cdecl("sk_keyframe_sequence_set_scalar")
public func sk_keyframe_sequence_set_scalar(_ sequenceHandle: UnsafeMutableRawPointer?, _ value: Double, _ index: Int) {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return }
    sequence.setKeyframeValue(NSNumber(value: value), for: index)
}

@_cdecl("sk_keyframe_sequence_set_time")
public func sk_keyframe_sequence_set_time(_ sequenceHandle: UnsafeMutableRawPointer?, _ time: Double, _ index: Int) {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return }
    sequence.setKeyframeTime(CGFloat(time), for: index)
}

@_cdecl("sk_keyframe_sequence_sample_scalar")
public func sk_keyframe_sequence_sample_scalar(_ sequenceHandle: UnsafeMutableRawPointer?, _ time: Double, _ outValue: UnsafeMutablePointer<Double>?) -> Bool {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle),
          let outValue,
          let value = sequence.sample(atTime: CGFloat(time)) as? NSNumber
    else { return false }
    outValue.pointee = value.doubleValue
    return true
}

@_cdecl("sk_keyframe_sequence_get_interpolation_mode")
public func sk_keyframe_sequence_get_interpolation_mode(_ sequenceHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return 1 }
    return Int32(sequence.interpolationMode.rawValue)
}

@_cdecl("sk_keyframe_sequence_set_interpolation_mode")
public func sk_keyframe_sequence_set_interpolation_mode(_ sequenceHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return }
    sequence.interpolationMode = SKInterpolationMode(rawValue: Int(mode)) ?? .linear
}

@_cdecl("sk_keyframe_sequence_get_repeat_mode")
public func sk_keyframe_sequence_get_repeat_mode(_ sequenceHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return 1 }
    return Int32(sequence.repeatMode.rawValue)
}

@_cdecl("sk_keyframe_sequence_set_repeat_mode")
public func sk_keyframe_sequence_set_repeat_mode(_ sequenceHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let sequence: SKKeyframeSequence = skBorrow(sequenceHandle) else { return }
    sequence.repeatMode = SKRepeatMode(rawValue: Int(mode)) ?? .clamp
}
