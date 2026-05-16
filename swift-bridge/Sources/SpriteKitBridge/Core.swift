import AppKit
import CoreGraphics
import Foundation
import Metal
import QuartzCore
import SpriteKit

@inline(__always)
func skRetain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func skReleaseHandle(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@inline(__always)
func skBorrow<T>(_ handle: UnsafeMutableRawPointer?) -> T? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? T
}

@inline(__always)
func skDup(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else { return nil }
    return strdup(value)
}

func skMakeColor(r: Float, g: Float, b: Float, a: Float) -> NSColor {
    NSColor(srgbRed: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: CGFloat(a))
}

func skLoadAction(_ rawValue: Int32) -> MTLLoadAction {
    switch rawValue {
    case 0: return .dontCare
    case 1: return .load
    default: return .clear
    }
}

func skStoreAction(_ rawValue: Int32) -> MTLStoreAction {
    switch rawValue {
    case 0: return .dontCare
    case 2: return .multisampleResolve
    default: return .store
    }
}

func skBlendMode(_ rawValue: Int32) -> SKBlendMode {
    SKBlendMode(rawValue: Int(rawValue)) ?? .alpha
}

func skHandles<T>(from rawHandles: UnsafeMutableRawPointer?, count: Int) -> [T] {
    guard let rawHandles else { return [] }
    let pointers = rawHandles.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    var objects: [T] = []
    objects.reserveCapacity(count)
    for index in 0..<count {
        if let object: T = skBorrow(pointers[index]) {
            objects.append(object)
        }
    }
    return objects
}

func skActions(from rawActions: UnsafeMutableRawPointer?, count: Int) -> [SKAction] {
    skHandles(from: rawActions, count: count)
}

func skTextures(from rawTextures: UnsafeMutableRawPointer?, count: Int) -> [SKTexture] {
    skHandles(from: rawTextures, count: count)
}

func skBodies(from rawBodies: UnsafeMutableRawPointer?, count: Int) -> [SKPhysicsBody] {
    skHandles(from: rawBodies, count: count)
}

func skConstraints(from rawConstraints: UnsafeMutableRawPointer?, count: Int) -> [SKConstraint] {
    skHandles(from: rawConstraints, count: count)
}

func skBorrowCGImage(_ handle: UnsafeMutableRawPointer?) -> CGImage? {
    guard let handle else { return nil }
    return Unmanaged<CGImage>.fromOpaque(handle).takeUnretainedValue()
}

@_cdecl("sk_release")
public func sk_release(_ handle: UnsafeMutableRawPointer?) {
    skReleaseHandle(handle)
}
