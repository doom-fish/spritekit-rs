import SpriteKit

public typealias MutableTextureModifyPixelDataCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    Int
) -> Void
public typealias TextureAtlasCompletionCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias TextureAtlasNamedCompletionCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<CChar>?,
    UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    Int
) -> Void

private func skTextureAtlasNames(from rawNames: UnsafeMutableRawPointer?, count: Int) -> [String] {
    guard let rawNames else { return [] }
    let pointers = rawNames.assumingMemoryBound(to: UnsafePointer<CChar>?.self)
    var names: [String] = []
    names.reserveCapacity(count)
    for index in 0..<count {
        if let name = pointers[index] {
            names.append(String(cString: name))
        }
    }
    return names
}

@_cdecl("sk_mutable_texture_new_with_size")
public func sk_mutable_texture_new_with_size(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKMutableTexture(size: CGSize(width: width, height: height)))
}

@_cdecl("sk_mutable_texture_new_with_size_pixel_format")
public func sk_mutable_texture_new_with_size_pixel_format(_ width: Double, _ height: Double, _ pixelFormat: Int32) -> UnsafeMutableRawPointer? {
    skRetain(SKMutableTexture(size: CGSize(width: width, height: height), pixelFormat: pixelFormat))
}

@_cdecl("sk_mutable_texture_modify_pixel_data")
public func sk_mutable_texture_modify_pixel_data(
    _ textureHandle: UnsafeMutableRawPointer?,
    _ context: UnsafeMutableRawPointer?,
    _ modify: MutableTextureModifyPixelDataCallback?
) {
    guard let texture: SKMutableTexture = skBorrow(textureHandle) else { return }
    texture.modifyPixelData { pixelData, length in
        modify?(context, pixelData, length)
    }
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

@_cdecl("sk_texture_atlas_preload_texture_atlases")
public func sk_texture_atlas_preload_texture_atlases(
    _ atlasHandles: UnsafeMutableRawPointer?,
    _ count: Int,
    _ context: UnsafeMutableRawPointer?,
    _ completion: TextureAtlasCompletionCallback?
) {
    let atlases: [SKTextureAtlas] = skHandles(from: atlasHandles, count: count)
    SKTextureAtlas.preloadTextureAtlases(atlases) {
        completion?(context)
    }
}

@_cdecl("sk_texture_atlas_preload_texture_atlases_named")
public func sk_texture_atlas_preload_texture_atlases_named(
    _ rawAtlasNames: UnsafeMutableRawPointer?,
    _ count: Int,
    _ context: UnsafeMutableRawPointer?,
    _ completion: TextureAtlasNamedCompletionCallback?
) {
    let atlasNames = skTextureAtlasNames(from: rawAtlasNames, count: count)
    SKTextureAtlas.preloadTextureAtlasesNamed(atlasNames) { error, atlases in
        guard let completion else { return }
        let errorMessage = skDup(error?.localizedDescription)
        var atlasHandles = atlases.map { Optional(skRetain($0)) }
        let atlasHandleCount = atlasHandles.count
        atlasHandles.withUnsafeMutableBufferPointer { buffer in
            completion(context, errorMessage, buffer.baseAddress, atlasHandleCount)
        }
    }
}

@_cdecl("sk_texture_atlas_preload")
public func sk_texture_atlas_preload(
    _ atlasHandle: UnsafeMutableRawPointer?,
    _ context: UnsafeMutableRawPointer?,
    _ completion: TextureAtlasCompletionCallback?
) {
    guard let atlas: SKTextureAtlas = skBorrow(atlasHandle) else { return }
    atlas.preload {
        completion?(context)
    }
}
