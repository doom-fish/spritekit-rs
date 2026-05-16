use core::ffi::{c_char, c_void};

extern "C" {
    pub fn sk_sprite_node_new_with_texture(texture: *mut c_void) -> *mut c_void;
    pub fn sk_sprite_node_new_with_color(r: f32, g: f32, b: f32, a: f32, width: f64, height: f64) -> *mut c_void;
    pub fn sk_sprite_node_new_image_named(name: *const c_char) -> *mut c_void;
    pub fn sk_sprite_node_get_texture(node: *mut c_void) -> *mut c_void;
    pub fn sk_sprite_node_set_texture(node: *mut c_void, texture: *mut c_void);
    pub fn sk_sprite_node_get_normal_texture(node: *mut c_void) -> *mut c_void;
    pub fn sk_sprite_node_set_normal_texture(node: *mut c_void, texture: *mut c_void);
    pub fn sk_sprite_node_get_size_w(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_get_size_h(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_set_size(node: *mut c_void, width: f64, height: f64);
    pub fn sk_sprite_node_scale_to_size(node: *mut c_void, width: f64, height: f64);
    pub fn sk_sprite_node_get_anchor_x(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_get_anchor_y(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_set_anchor_point(node: *mut c_void, x: f64, y: f64);
    pub fn sk_sprite_node_set_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn sk_sprite_node_get_color_blend_factor(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_set_color_blend_factor(node: *mut c_void, factor: f64);
    pub fn sk_sprite_node_get_blend_mode(node: *mut c_void) -> i32;
    pub fn sk_sprite_node_set_blend_mode(node: *mut c_void, mode: i32);
    pub fn sk_sprite_node_get_lighting_bitmask(node: *mut c_void) -> u32;
    pub fn sk_sprite_node_set_lighting_bitmask(node: *mut c_void, mask: u32);
    pub fn sk_sprite_node_get_shadow_cast_bitmask(node: *mut c_void) -> u32;
    pub fn sk_sprite_node_set_shadow_cast_bitmask(node: *mut c_void, mask: u32);
    pub fn sk_sprite_node_get_shadowed_bitmask(node: *mut c_void) -> u32;
    pub fn sk_sprite_node_set_shadowed_bitmask(node: *mut c_void, mask: u32);
    pub fn sk_sprite_node_get_center_rect_x(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_get_center_rect_y(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_get_center_rect_w(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_get_center_rect_h(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_set_center_rect(node: *mut c_void, x: f64, y: f64, width: f64, height: f64);
    pub fn sk_sprite_node_get_shader(node: *mut c_void) -> *mut c_void;
    pub fn sk_sprite_node_set_shader(node: *mut c_void, shader: *mut c_void);
}
