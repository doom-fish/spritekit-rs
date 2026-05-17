use core::ffi::{c_char, c_void};

extern "C" {
    pub fn sk_mutable_texture_new_with_size(width: f64, height: f64) -> *mut c_void;
    pub fn sk_mutable_texture_new_with_size_pixel_format(
        width: f64,
        height: f64,
        pixel_format: i32,
    ) -> *mut c_void;

    pub fn sk_texture_atlas_named(name: *const c_char) -> *mut c_void;
    pub fn sk_texture_atlas_get_texture_names_count(atlas: *mut c_void) -> usize;
    pub fn sk_texture_atlas_texture_named(atlas: *mut c_void, name: *const c_char) -> *mut c_void;
}
