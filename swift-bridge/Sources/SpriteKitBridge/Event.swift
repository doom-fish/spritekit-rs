import AppKit
import SpriteKit

@_cdecl("sk_event_mouse_moved")
public func sk_event_mouse_moved(_ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    guard let event = NSEvent.mouseEvent(
        with: .mouseMoved,
        location: NSPoint(x: x, y: y),
        modifierFlags: [],
        timestamp: 0,
        windowNumber: 0,
        context: nil,
        eventNumber: 0,
        clickCount: 0,
        pressure: 0
    ) else {
        return nil
    }
    return skRetain(event)
}

@_cdecl("sk_event_location_in_node_x")
public func sk_event_location_in_node_x(_ eventHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let event: NSEvent = skBorrow(eventHandle),
          let node: SKNode = skBorrow(nodeHandle)
    else { return 0 }
    return Double(event.location(in: node).x)
}

@_cdecl("sk_event_location_in_node_y")
public func sk_event_location_in_node_y(_ eventHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let event: NSEvent = skBorrow(eventHandle),
          let node: SKNode = skBorrow(nodeHandle)
    else { return 0 }
    return Double(event.location(in: node).y)
}
