use core::ffi::{c_char, c_void};

extern "C" {
    pub fn sk_shader_new() -> *mut c_void;
    pub fn sk_shader_new_with_source(source: *const c_char) -> *mut c_void;
    pub fn sk_shader_copy_source(shader: *mut c_void) -> *mut c_char;
    pub fn sk_shader_set_source(shader: *mut c_void, source: *const c_char);
    pub fn sk_shader_uniform_count(shader: *mut c_void) -> usize;
    pub fn sk_shader_add_float_uniform(shader: *mut c_void, name: *const c_char, value: f32);
    pub fn sk_shader_get_float_uniform(shader: *mut c_void, name: *const c_char, out_value: *mut f32) -> bool;
    pub fn sk_shader_remove_uniform_named(shader: *mut c_void, name: *const c_char);
    pub fn sk_shader_attribute_count(shader: *mut c_void) -> usize;
}
