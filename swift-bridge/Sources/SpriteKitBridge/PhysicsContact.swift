import SpriteKit

public typealias PhysicsContactCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
public typealias PhysicsContactReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class PhysicsContactDelegateBridge: NSObject, SKPhysicsContactDelegate {
    private let context: UnsafeMutableRawPointer?
    private let didBeginCallback: PhysicsContactCallback?
    private let didEndCallback: PhysicsContactCallback?
    private let releaseCallback: PhysicsContactReleaseCallback?

    init(
        context: UnsafeMutableRawPointer?,
        didBeginCallback: PhysicsContactCallback?,
        didEndCallback: PhysicsContactCallback?,
        releaseCallback: PhysicsContactReleaseCallback?
    ) {
        self.context = context
        self.didBeginCallback = didBeginCallback
        self.didEndCallback = didEndCallback
        self.releaseCallback = releaseCallback
    }

    deinit {
        releaseCallback?(context)
    }

    func didBegin(_ contact: SKPhysicsContact) {
        didBeginCallback?(context, skRetain(contact))
    }

    func didEnd(_ contact: SKPhysicsContact) {
        didEndCallback?(context, skRetain(contact))
    }
}

@_cdecl("sk_physics_contact_get_body_a")
public func sk_physics_contact_get_body_a(_ contactHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contact: SKPhysicsContact = skBorrow(contactHandle) else { return nil }
    return skRetain(contact.bodyA)
}

@_cdecl("sk_physics_contact_get_body_b")
public func sk_physics_contact_get_body_b(_ contactHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contact: SKPhysicsContact = skBorrow(contactHandle) else { return nil }
    return skRetain(contact.bodyB)
}

@_cdecl("sk_physics_contact_get_contact_point_x")
public func sk_physics_contact_get_contact_point_x(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SKPhysicsContact = skBorrow(contactHandle) else { return 0 }
    return Double(contact.contactPoint.x)
}

@_cdecl("sk_physics_contact_get_contact_point_y")
public func sk_physics_contact_get_contact_point_y(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SKPhysicsContact = skBorrow(contactHandle) else { return 0 }
    return Double(contact.contactPoint.y)
}

@_cdecl("sk_physics_contact_get_contact_normal_dx")
public func sk_physics_contact_get_contact_normal_dx(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SKPhysicsContact = skBorrow(contactHandle) else { return 0 }
    return Double(contact.contactNormal.dx)
}

@_cdecl("sk_physics_contact_get_contact_normal_dy")
public func sk_physics_contact_get_contact_normal_dy(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SKPhysicsContact = skBorrow(contactHandle) else { return 0 }
    return Double(contact.contactNormal.dy)
}

@_cdecl("sk_physics_contact_get_collision_impulse")
public func sk_physics_contact_get_collision_impulse(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SKPhysicsContact = skBorrow(contactHandle) else { return 0 }
    return Double(contact.collisionImpulse)
}

@_cdecl("sk_physics_contact_delegate_new")
public func sk_physics_contact_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ didBegin: PhysicsContactCallback?,
    _ didEnd: PhysicsContactCallback?,
    _ releaseContext: PhysicsContactReleaseCallback?
) -> UnsafeMutableRawPointer? {
    skRetain(PhysicsContactDelegateBridge(
        context: context,
        didBeginCallback: didBegin,
        didEndCallback: didEnd,
        releaseCallback: releaseContext
    ))
}

@_cdecl("sk_physics_world_set_contact_delegate")
public func sk_physics_world_set_contact_delegate(_ worldHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return }
    let delegate: PhysicsContactDelegateBridge? = skBorrow(delegateHandle)
    world.contactDelegate = delegate
}

@_cdecl("sk_physics_world_has_contact_delegate")
public func sk_physics_world_has_contact_delegate(_ worldHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let world: SKPhysicsWorld = skBorrow(worldHandle) else { return false }
    return world.contactDelegate != nil
}
