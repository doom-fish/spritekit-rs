import SpriteKit

@_cdecl("sk_mutable_texture_new_with_size")
public func sk_mutable_texture_new_with_size(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKMutableTexture(size: CGSize(width: width, height: height)))
}

@_cdecl("sk_mutable_texture_new_with_size_pixel_format")
public func sk_mutable_texture_new_with_size_pixel_format(_ width: Double, _ height: Double, _ pixelFormat: Int32) -> UnsafeMutableRawPointer? {
    skRetain(SKMutableTexture(size: CGSize(width: width, height: height), pixelFormat: pixelFormat))
}

@_cdecl("sk_texture_atlas_named")
public func sk_texture_atlas_named(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    return skRetain(SKTextureAtlas(named: String(cString: name)))
}

@_cdecl("sk_texture_atlas_get_texture_names_count")
public func sk_texture_atlas_get_texture_names_count(_ atlasHandle: UnsafeMutableRawPointer?) -> Int {
    guard let atlas: SKTextureAtlas = skBorrow(atlasHandle) else { return 0 }
    return atlas.textureNames.count
}

@_cdecl("sk_texture_atlas_texture_named")
public func sk_texture_atlas_texture_named(_ atlasHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let atlas: SKTextureAtlas = skBorrow(atlasHandle),
          let name
    else { return nil }
    return skRetain(atlas.textureNamed(String(cString: name)))
}
