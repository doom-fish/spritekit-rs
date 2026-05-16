use core::ffi::c_void;

extern "C" {
    pub fn sk_light_node_new() -> *mut c_void;
    pub fn sk_light_node_get_enabled(node: *mut c_void) -> bool;
    pub fn sk_light_node_set_enabled(node: *mut c_void, enabled: bool);
    pub fn sk_light_node_set_light_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn sk_light_node_set_ambient_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn sk_light_node_set_shadow_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn sk_light_node_get_falloff(node: *mut c_void) -> f64;
    pub fn sk_light_node_set_falloff(node: *mut c_void, falloff: f64);
    pub fn sk_light_node_get_category_bitmask(node: *mut c_void) -> u32;
    pub fn sk_light_node_set_category_bitmask(node: *mut c_void, mask: u32);
}
