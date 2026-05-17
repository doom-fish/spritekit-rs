import SpriteKit

public typealias SceneUpdateCallback = @convention(c) (UnsafeMutableRawPointer?, Double) -> Void
public typealias SceneReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class SceneDelegateBridge: NSObject, SKSceneDelegate {
    private let context: UnsafeMutableRawPointer?
    private let updateCallback: SceneUpdateCallback?
    private let releaseCallback: SceneReleaseCallback?

    init(context: UnsafeMutableRawPointer?, updateCallback: SceneUpdateCallback?, releaseCallback: SceneReleaseCallback?) {
        self.context = context
        self.updateCallback = updateCallback
        self.releaseCallback = releaseCallback
    }

    deinit {
        releaseCallback?(context)
    }

    func update(_ currentTime: TimeInterval, for scene: SKScene) {
        updateCallback?(context, currentTime)
    }
}

@_cdecl("sk_scene_new_with_size")
public func sk_scene_new_with_size(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    skRetain(SKScene(size: CGSize(width: width, height: height)))
}

@_cdecl("sk_scene_get_size_w")
public func sk_scene_get_size_w(_ sceneHandle: UnsafeMutableRawPointer?) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.size.width)
}

@_cdecl("sk_scene_get_size_h")
public func sk_scene_get_size_h(_ sceneHandle: UnsafeMutableRawPointer?) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.size.height)
}

@_cdecl("sk_scene_set_size")
public func sk_scene_set_size(_ sceneHandle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return }
    scene.size = CGSize(width: width, height: height)
}

@_cdecl("sk_scene_get_scale_mode")
public func sk_scene_get_scale_mode(_ sceneHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Int32(scene.scaleMode.rawValue)
}

@_cdecl("sk_scene_set_scale_mode")
public func sk_scene_set_scale_mode(_ sceneHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return }
    scene.scaleMode = SKSceneScaleMode(rawValue: Int(mode)) ?? .aspectFit
}

@_cdecl("sk_scene_set_background_color")
public func sk_scene_set_background_color(
    _ sceneHandle: UnsafeMutableRawPointer?,
    _ r: Float, _ g: Float, _ b: Float, _ a: Float
) {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return }
    scene.backgroundColor = skMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("sk_scene_get_camera")
public func sk_scene_get_camera(_ sceneHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scene: SKScene = skBorrow(sceneHandle),
          let camera = scene.camera
    else { return nil }
    return skRetain(camera)
}

@_cdecl("sk_scene_set_camera")
public func sk_scene_set_camera(_ sceneHandle: UnsafeMutableRawPointer?, _ cameraHandle: UnsafeMutableRawPointer?) {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return }
    let camera: SKCameraNode? = skBorrow(cameraHandle)
    scene.camera = camera
}

@_cdecl("sk_scene_delegate_new")
public func sk_scene_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ update: SceneUpdateCallback?,
    _ releaseContext: SceneReleaseCallback?
) -> UnsafeMutableRawPointer? {
    skRetain(SceneDelegateBridge(context: context, updateCallback: update, releaseCallback: releaseContext))
}

@_cdecl("sk_scene_set_delegate")
public func sk_scene_set_delegate(_ sceneHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return }
    let delegate: SceneDelegateBridge? = skBorrow(delegateHandle)
    scene.delegate = delegate
}

@_cdecl("sk_scene_has_delegate")
public func sk_scene_has_delegate(_ sceneHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return false }
    return scene.delegate != nil
}

@_cdecl("sk_scene_get_anchor_x")
public func sk_scene_get_anchor_x(_ sceneHandle: UnsafeMutableRawPointer?) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.anchorPoint.x)
}

@_cdecl("sk_scene_get_anchor_y")
public func sk_scene_get_anchor_y(_ sceneHandle: UnsafeMutableRawPointer?) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.anchorPoint.y)
}

@_cdecl("sk_scene_set_anchor_point")
public func sk_scene_set_anchor_point(_ sceneHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return }
    scene.anchorPoint = CGPoint(x: x, y: y)
}

@_cdecl("sk_scene_physics_world")
public func sk_scene_physics_world(_ sceneHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return nil }
    return skRetain(scene.physicsWorld)
}

@_cdecl("sk_scene_get_view")
public func sk_scene_get_view(_ sceneHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scene: SKScene = skBorrow(sceneHandle), let view = scene.view else { return nil }
    return skRetain(view)
}

@_cdecl("sk_scene_convert_point_from_view_x")
public func sk_scene_convert_point_from_view_x(_ sceneHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.convertPoint(fromView: CGPoint(x: x, y: y)).x)
}

@_cdecl("sk_scene_convert_point_from_view_y")
public func sk_scene_convert_point_from_view_y(_ sceneHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.convertPoint(fromView: CGPoint(x: x, y: y)).y)
}

@_cdecl("sk_scene_convert_point_to_view_x")
public func sk_scene_convert_point_to_view_x(_ sceneHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.convertPoint(toView: CGPoint(x: x, y: y)).x)
}

@_cdecl("sk_scene_convert_point_to_view_y")
public func sk_scene_convert_point_to_view_y(_ sceneHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> Double {
    guard let scene: SKScene = skBorrow(sceneHandle) else { return 0 }
    return Double(scene.convertPoint(toView: CGPoint(x: x, y: y)).y)
}
