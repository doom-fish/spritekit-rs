use core::ffi::c_void;

extern "C" {
    /// Wraps `SKAction`.
    pub fn sk_action_move_by(dx: f64, dy: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_move_to(x: f64, y: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_rotate_by(angle: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_rotate_to(angle: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_scale_by(scale: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_scale_to(scale: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_resize_to(width: f64, height: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_fade_in(duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_fade_out(duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_fade_to(alpha: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_fade_alpha_by(alpha: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_hide() -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_unhide() -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_set_texture(texture: *mut c_void) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_animate_with_textures(
        textures: *mut c_void,
        count: usize,
        time_per_frame: f64,
    ) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_wait(duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_sequence(actions: *mut c_void, count: usize) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_group(actions: *mut c_void, count: usize) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_repeat(action: *mut c_void, count: usize) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_repeat_forever(action: *mut c_void) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_change_charge_to(value: f32, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_apply_force(dx: f64, dy: f64, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_play() -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_change_volume_to(value: f32, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_stereo_pan_to(value: f32, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_warp_to(warp: *mut c_void, duration: f64) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_animate_with_warps(
        warps: *mut c_void,
        times: *const f64,
        count: usize,
    ) -> *mut c_void;
    /// Wraps `SKAction`.
    pub fn sk_action_get_duration(action: *mut c_void) -> f64;
    /// Wraps `SKAction`.
    pub fn sk_action_set_duration(action: *mut c_void, duration: f64);
    /// Wraps `SKAction`.
    pub fn sk_action_get_timing_mode(action: *mut c_void) -> i32;
    /// Wraps `SKAction`.
    pub fn sk_action_set_timing_mode(action: *mut c_void, mode: i32);
    /// Wraps `SKAction`.
    pub fn sk_action_get_speed(action: *mut c_void) -> f64;
    /// Wraps `SKAction`.
    pub fn sk_action_set_speed(action: *mut c_void, speed: f64);
    /// Wraps `SKAction`.
    pub fn sk_action_reversed(action: *mut c_void) -> *mut c_void;
}
