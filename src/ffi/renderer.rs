use core::ffi::c_void;

extern "C" {
    pub fn sk_render_pass_descriptor_new_for_texture(texture: *mut c_void, clear_r: f64, clear_g: f64, clear_b: f64, clear_a: f64, load_action: i32, store_action: i32) -> *mut c_void;
    pub fn sk_renderer_new(device: *mut c_void) -> *mut c_void;
    pub fn sk_renderer_set_scene(renderer: *mut c_void, scene: *mut c_void);
    pub fn sk_renderer_update_at_time(renderer: *mut c_void, time: f64);
    pub fn sk_renderer_render(renderer: *mut c_void, vp_x: f64, vp_y: f64, vp_w: f64, vp_h: f64, command_buffer: *mut c_void, pass_descriptor: *mut c_void);
    pub fn sk_texture_copy_bytes(texture: *mut c_void, out_bytes: *mut c_void, bytes_per_row: usize) -> bool;
}
