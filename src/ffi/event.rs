use core::ffi::c_void;

extern "C" {
    /// Wraps `NSEvent` in `SpriteKit` input handling.
    pub fn sk_event_mouse_moved(x: f64, y: f64) -> *mut c_void;
    /// Wraps `NSEvent` in `SpriteKit` input handling.
    pub fn sk_event_location_in_node_x(event: *mut c_void, node: *mut c_void) -> f64;
    /// Wraps `NSEvent` in `SpriteKit` input handling.
    pub fn sk_event_location_in_node_y(event: *mut c_void, node: *mut c_void) -> f64;
}
