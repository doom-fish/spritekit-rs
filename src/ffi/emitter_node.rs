use core::ffi::c_void;

extern "C" {
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_new() -> *mut c_void;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_advance_simulation_time(node: *mut c_void, seconds: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_reset_simulation(node: *mut c_void);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_texture(node: *mut c_void) -> *mut c_void;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_texture(node: *mut c_void, texture: *mut c_void);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_blend_mode(node: *mut c_void) -> i32;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_blend_mode(node: *mut c_void, mode: i32);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_render_order(node: *mut c_void) -> u64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_render_order(node: *mut c_void, order: u64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_position_x(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_position_y(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_position(node: *mut c_void, x: f64, y: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_position_range_dx(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_position_range_dy(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_position_range(node: *mut c_void, dx: f64, dy: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_speed(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_speed(node: *mut c_void, speed: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_speed_range(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_speed_range(node: *mut c_void, range: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_emission_angle(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_emission_angle(node: *mut c_void, angle: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_emission_angle_range(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_emission_angle_range(node: *mut c_void, range: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_x_acceleration(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_x_acceleration(node: *mut c_void, acceleration: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_y_acceleration(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_y_acceleration(node: *mut c_void, acceleration: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_birth_rate(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_birth_rate(node: *mut c_void, birth_rate: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_num_particles_to_emit(node: *mut c_void) -> usize;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_num_particles_to_emit(node: *mut c_void, count: usize);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_lifetime(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_lifetime(node: *mut c_void, lifetime: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_lifetime_range(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_lifetime_range(node: *mut c_void, range: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_rotation(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_rotation(node: *mut c_void, rotation: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_rotation_range(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_rotation_range(node: *mut c_void, range: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_rotation_speed(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_rotation_speed(node: *mut c_void, speed: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_size_w(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_size_h(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_size(node: *mut c_void, width: f64, height: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_scale(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_scale(node: *mut c_void, scale: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_scale_range(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_scale_range(node: *mut c_void, range: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_scale_speed(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_scale_speed(node: *mut c_void, speed: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_alpha(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_alpha(node: *mut c_void, alpha: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_alpha_range(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_alpha_range(node: *mut c_void, range: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_particle_alpha_speed(node: *mut c_void) -> f64;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_particle_alpha_speed(node: *mut c_void, speed: f64);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_get_field_bitmask(node: *mut c_void) -> u32;
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_field_bitmask(node: *mut c_void, mask: u32);
    /// Wraps `SKEmitterNode`.
    pub fn sk_emitter_node_set_target_node(node: *mut c_void, target_node: *mut c_void);
}
