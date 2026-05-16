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

func skActions(from rawActions: UnsafeMutableRawPointer?, count: Int) -> [SKAction] {
    guard let rawActions else { return [] }
    let pointers = rawActions.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    var actions: [SKAction] = []
    actions.reserveCapacity(count)
    for index in 0..<count {
        if let action: SKAction = skBorrow(pointers[index]) {
            actions.append(action)
        }
    }
    return actions
}

func skTextures(from rawTextures: UnsafeMutableRawPointer?, count: Int) -> [SKTexture] {
    guard let rawTextures else { return [] }
    let pointers = rawTextures.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    var textures: [SKTexture] = []
    textures.reserveCapacity(count)
    for index in 0..<count {
        if let texture: SKTexture = skBorrow(pointers[index]) {
            textures.append(texture)
        }
    }
    return textures
}

func skBorrowCGImage(_ handle: UnsafeMutableRawPointer?) -> CGImage? {
    guard let handle else { return nil }
    return Unmanaged<CGImage>.fromOpaque(handle).takeUnretainedValue()
}

@_cdecl("sk_release")
public func sk_release(_ handle: UnsafeMutableRawPointer?) {
    skReleaseHandle(handle)
}
