use core::ffi::c_void;

extern "C" {
    /// Wraps retained `SpriteKit` object handles.
    pub fn sk_release(handle: *mut c_void);
}
