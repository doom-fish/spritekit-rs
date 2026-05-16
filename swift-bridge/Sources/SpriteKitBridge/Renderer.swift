import Metal
import SpriteKit

final class SpriteKitRenderPassDescriptorBox: NSObject {
    let descriptor: MTLRenderPassDescriptor

    init(
        texture: MTLTexture,
        clearColor: MTLClearColor,
        loadAction: MTLLoadAction,
        storeAction: MTLStoreAction
    ) {
        let descriptor = MTLRenderPassDescriptor()
        descriptor.colorAttachments[0].texture = texture
        descriptor.colorAttachments[0].loadAction = loadAction
        descriptor.colorAttachments[0].storeAction = storeAction
        descriptor.colorAttachments[0].clearColor = clearColor
        self.descriptor = descriptor
    }
}

func skBorrowRenderPassDescriptor(_ handle: UnsafeMutableRawPointer?) -> MTLRenderPassDescriptor? {
    guard let box: SpriteKitRenderPassDescriptorBox = skBorrow(handle) else { return nil }
    return box.descriptor
}

@_cdecl("sk_render_pass_descriptor_new_for_texture")
public func sk_render_pass_descriptor_new_for_texture(
    _ textureHandle: UnsafeMutableRawPointer?,
    _ clearR: Double, _ clearG: Double, _ clearB: Double, _ clearA: Double,
    _ loadAction: Int32,
    _ storeAction: Int32
) -> UnsafeMutableRawPointer? {
    guard let texture: MTLTexture = skBorrow(textureHandle) else { return nil }
    let box = SpriteKitRenderPassDescriptorBox(
        texture: texture,
        clearColor: MTLClearColor(red: clearR, green: clearG, blue: clearB, alpha: clearA),
        loadAction: skLoadAction(loadAction),
        storeAction: skStoreAction(storeAction)
    )
    return skRetain(box)
}

@_cdecl("sk_renderer_new")
public func sk_renderer_new(_ deviceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let device: MTLDevice? = skBorrow(deviceHandle)
    guard let device else { return nil }
    return skRetain(SKRenderer(device: device))
}

@_cdecl("sk_renderer_set_scene")
public func sk_renderer_set_scene(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ sceneHandle: UnsafeMutableRawPointer?
) {
    guard let renderer: SKRenderer = skBorrow(rendererHandle) else { return }
    let scene: SKScene? = skBorrow(sceneHandle)
    renderer.scene = scene
}

@_cdecl("sk_renderer_update_at_time")
public func sk_renderer_update_at_time(_ rendererHandle: UnsafeMutableRawPointer?, _ time: Double) {
    guard let renderer: SKRenderer = skBorrow(rendererHandle) else { return }
    renderer.update(atTime: time)
}

@_cdecl("sk_renderer_render")
public func sk_renderer_render(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ vpX: Double, _ vpY: Double, _ vpW: Double, _ vpH: Double,
    _ commandBufferHandle: UnsafeMutableRawPointer?,
    _ passDescriptorHandle: UnsafeMutableRawPointer?
) {
    guard let renderer: SKRenderer = skBorrow(rendererHandle),
          let commandBuffer: MTLCommandBuffer = skBorrow(commandBufferHandle),
          let passDescriptor = skBorrowRenderPassDescriptor(passDescriptorHandle)
    else { return }
    renderer.render(
        withViewport: CGRect(x: vpX, y: vpY, width: vpW, height: vpH),
        commandBuffer: commandBuffer,
        renderPassDescriptor: passDescriptor
    )
}

@_cdecl("sk_texture_copy_bytes")
public func sk_texture_copy_bytes(
    _ textureHandle: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutableRawPointer?,
    _ bytesPerRow: Int
) -> Bool {
    guard let texture: MTLTexture = skBorrow(textureHandle), let outBytes else { return false }
    let region = MTLRegionMake2D(0, 0, texture.width, texture.height)
    texture.getBytes(outBytes, bytesPerRow: bytesPerRow, from: region, mipmapLevel: 0)
    return true
}
