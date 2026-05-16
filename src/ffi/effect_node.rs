use core::ffi::c_void;

extern "C" {
    pub fn sk_effect_node_new() -> *mut c_void;
    pub fn sk_effect_node_get_should_enable_effects(node: *mut c_void) -> bool;
    pub fn sk_effect_node_set_should_enable_effects(node: *mut c_void, enable: bool);
    pub fn sk_effect_node_get_should_rasterize(node: *mut c_void) -> bool;
    pub fn sk_effect_node_set_should_rasterize(node: *mut c_void, rasterize: bool);
    pub fn sk_effect_node_get_blend_mode(node: *mut c_void) -> i32;
    pub fn sk_effect_node_set_blend_mode(node: *mut c_void, mode: i32);
}
