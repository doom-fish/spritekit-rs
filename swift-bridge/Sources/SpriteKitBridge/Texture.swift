import CoreGraphics
import SpriteKit

@_cdecl("sk_texture_image_named")
public func sk_texture_image_named(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    return skRetain(SKTexture(imageNamed: String(cString: name)))
}

@_cdecl("sk_texture_from_cg_image")
public func sk_texture_from_cg_image(_ imageHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let image = skBorrowCGImage(imageHandle) else { return nil }
    return skRetain(SKTexture(cgImage: image))
}

@_cdecl("sk_texture_from_rgba_bytes")
public func sk_texture_from_rgba_bytes(
    _ bytes: UnsafeRawPointer?,
    _ length: Int,
    _ width: Int,
    _ height: Int
) -> UnsafeMutableRawPointer? {
    guard let bytes else { return nil }
    let data = Data(bytes: bytes, count: length)
    return skRetain(SKTexture(data: data, size: CGSize(width: width, height: height)))
}

@_cdecl("sk_texture_subrect")
public func sk_texture_subrect(
    _ textureHandle: UnsafeMutableRawPointer?,
    _ x: Double, _ y: Double, _ width: Double, _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return nil }
    let rect = CGRect(x: x, y: y, width: width, height: height)
    return skRetain(SKTexture(rect: rect, in: texture))
}

@_cdecl("sk_texture_get_size_w")
public func sk_texture_get_size_w(_ textureHandle: UnsafeMutableRawPointer?) -> Double {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return 0 }
    return Double(texture.size().width)
}

@_cdecl("sk_texture_get_size_h")
public func sk_texture_get_size_h(_ textureHandle: UnsafeMutableRawPointer?) -> Double {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return 0 }
    return Double(texture.size().height)
}

@_cdecl("sk_texture_get_filtering_mode")
public func sk_texture_get_filtering_mode(_ textureHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return 0 }
    return Int32(texture.filteringMode.rawValue)
}

@_cdecl("sk_texture_set_filtering_mode")
public func sk_texture_set_filtering_mode(_ textureHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return }
    texture.filteringMode = SKTextureFilteringMode(rawValue: Int(mode)) ?? .linear
}

@_cdecl("sk_texture_get_uses_mipmaps")
public func sk_texture_get_uses_mipmaps(_ textureHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return false }
    return texture.usesMipmaps
}

@_cdecl("sk_texture_set_uses_mipmaps")
public func sk_texture_set_uses_mipmaps(_ textureHandle: UnsafeMutableRawPointer?, _ uses: Bool) {
    guard let texture: SKTexture = skBorrow(textureHandle) else { return }
    texture.usesMipmaps = uses
}
