use core::ffi::c_void;

extern "C" {
    pub fn sk_release(handle: *mut c_void);
}
