use apple_cf::cg::{CGPoint, CGSize, CGVector};

use crate::color::Color;
use crate::ffi;
use crate::node::AsNode;
use crate::physics::BlendMode;
use crate::private::handle_type;
use crate::texture::Texture;

handle_type!(EmitterNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u64)]
pub enum ParticleRenderOrder {
    #[default]
    OldestLast = 0,
    OldestFirst = 1,
    DontCare = 2,
}

impl ParticleRenderOrder {
    #[must_use]
    pub const fn from_raw(value: u64) -> Self {
        match value {
            1 => Self::OldestFirst,
            2 => Self::DontCare,
            _ => Self::OldestLast,
        }
    }
}

impl AsNode for EmitterNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl EmitterNode {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_emitter_node_new()) }
    }

    pub fn advance_simulation_time(&self, seconds: f64) {
        unsafe { ffi::sk_emitter_node_advance_simulation_time(self.ptr, seconds) };
    }

    pub fn reset_simulation(&self) {
        unsafe { ffi::sk_emitter_node_reset_simulation(self.ptr) };
    }

    #[must_use]
    pub fn particle_texture(&self) -> Option<Texture> {
        unsafe { Texture::from_raw(ffi::sk_emitter_node_get_particle_texture(self.ptr)) }
    }

    pub fn set_particle_texture(&self, texture: Option<&Texture>) {
        unsafe {
            ffi::sk_emitter_node_set_particle_texture(
                self.ptr,
                texture.map_or(core::ptr::null_mut(), Texture::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn particle_blend_mode(&self) -> BlendMode {
        BlendMode::from_raw(unsafe { ffi::sk_emitter_node_get_particle_blend_mode(self.ptr) })
    }

    pub fn set_particle_blend_mode(&self, mode: BlendMode) {
        unsafe { ffi::sk_emitter_node_set_particle_blend_mode(self.ptr, mode as i32) };
    }

    #[must_use]
    pub fn particle_render_order(&self) -> ParticleRenderOrder {
        ParticleRenderOrder::from_raw(unsafe {
            ffi::sk_emitter_node_get_particle_render_order(self.ptr)
        })
    }

    pub fn set_particle_render_order(&self, order: ParticleRenderOrder) {
        unsafe { ffi::sk_emitter_node_set_particle_render_order(self.ptr, order as u64) };
    }

    pub fn set_particle_color(&self, color: Color) {
        unsafe {
            ffi::sk_emitter_node_set_particle_color(self.ptr, color.r, color.g, color.b, color.a)
        };
    }

    #[must_use]
    pub fn particle_position(&self) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_emitter_node_get_particle_position_x(self.ptr) },
            unsafe { ffi::sk_emitter_node_get_particle_position_y(self.ptr) },
        )
    }

    pub fn set_particle_position(&self, position: CGPoint) {
        unsafe { ffi::sk_emitter_node_set_particle_position(self.ptr, position.x, position.y) };
    }

    #[must_use]
    pub fn particle_position_range(&self) -> CGVector {
        CGVector::new(
            unsafe { ffi::sk_emitter_node_get_particle_position_range_dx(self.ptr) },
            unsafe { ffi::sk_emitter_node_get_particle_position_range_dy(self.ptr) },
        )
    }

    pub fn set_particle_position_range(&self, range: CGVector) {
        unsafe { ffi::sk_emitter_node_set_particle_position_range(self.ptr, range.dx, range.dy) };
    }

    #[must_use]
    pub fn particle_speed(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_speed(self.ptr) }
    }

    pub fn set_particle_speed(&self, speed: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_speed(self.ptr, speed) };
    }

    #[must_use]
    pub fn particle_speed_range(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_speed_range(self.ptr) }
    }

    pub fn set_particle_speed_range(&self, range: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_speed_range(self.ptr, range) };
    }

    #[must_use]
    pub fn emission_angle(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_emission_angle(self.ptr) }
    }

    pub fn set_emission_angle(&self, angle: f64) {
        unsafe { ffi::sk_emitter_node_set_emission_angle(self.ptr, angle) };
    }

    #[must_use]
    pub fn emission_angle_range(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_emission_angle_range(self.ptr) }
    }

    pub fn set_emission_angle_range(&self, range: f64) {
        unsafe { ffi::sk_emitter_node_set_emission_angle_range(self.ptr, range) };
    }

    #[must_use]
    pub fn x_acceleration(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_x_acceleration(self.ptr) }
    }

    pub fn set_x_acceleration(&self, acceleration: f64) {
        unsafe { ffi::sk_emitter_node_set_x_acceleration(self.ptr, acceleration) };
    }

    #[must_use]
    pub fn y_acceleration(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_y_acceleration(self.ptr) }
    }

    pub fn set_y_acceleration(&self, acceleration: f64) {
        unsafe { ffi::sk_emitter_node_set_y_acceleration(self.ptr, acceleration) };
    }

    #[must_use]
    pub fn particle_birth_rate(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_birth_rate(self.ptr) }
    }

    pub fn set_particle_birth_rate(&self, birth_rate: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_birth_rate(self.ptr, birth_rate) };
    }

    #[must_use]
    pub fn num_particles_to_emit(&self) -> usize {
        unsafe { ffi::sk_emitter_node_get_num_particles_to_emit(self.ptr) }
    }

    pub fn set_num_particles_to_emit(&self, count: usize) {
        unsafe { ffi::sk_emitter_node_set_num_particles_to_emit(self.ptr, count) };
    }

    #[must_use]
    pub fn particle_lifetime(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_lifetime(self.ptr) }
    }

    pub fn set_particle_lifetime(&self, lifetime: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_lifetime(self.ptr, lifetime) };
    }

    #[must_use]
    pub fn particle_lifetime_range(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_lifetime_range(self.ptr) }
    }

    pub fn set_particle_lifetime_range(&self, range: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_lifetime_range(self.ptr, range) };
    }

    #[must_use]
    pub fn particle_rotation(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_rotation(self.ptr) }
    }

    pub fn set_particle_rotation(&self, rotation: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_rotation(self.ptr, rotation) };
    }

    #[must_use]
    pub fn particle_rotation_range(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_rotation_range(self.ptr) }
    }

    pub fn set_particle_rotation_range(&self, range: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_rotation_range(self.ptr, range) };
    }

    #[must_use]
    pub fn particle_rotation_speed(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_rotation_speed(self.ptr) }
    }

    pub fn set_particle_rotation_speed(&self, speed: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_rotation_speed(self.ptr, speed) };
    }

    #[must_use]
    pub fn particle_size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_emitter_node_get_particle_size_w(self.ptr) },
            unsafe { ffi::sk_emitter_node_get_particle_size_h(self.ptr) },
        )
    }

    pub fn set_particle_size(&self, size: CGSize) {
        unsafe { ffi::sk_emitter_node_set_particle_size(self.ptr, size.width, size.height) };
    }

    #[must_use]
    pub fn particle_scale(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_scale(self.ptr) }
    }

    pub fn set_particle_scale(&self, scale: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_scale(self.ptr, scale) };
    }

    #[must_use]
    pub fn particle_scale_range(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_scale_range(self.ptr) }
    }

    pub fn set_particle_scale_range(&self, range: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_scale_range(self.ptr, range) };
    }

    #[must_use]
    pub fn particle_scale_speed(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_scale_speed(self.ptr) }
    }

    pub fn set_particle_scale_speed(&self, speed: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_scale_speed(self.ptr, speed) };
    }

    #[must_use]
    pub fn particle_alpha(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_alpha(self.ptr) }
    }

    pub fn set_particle_alpha(&self, alpha: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_alpha(self.ptr, alpha) };
    }

    #[must_use]
    pub fn particle_alpha_range(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_alpha_range(self.ptr) }
    }

    pub fn set_particle_alpha_range(&self, range: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_alpha_range(self.ptr, range) };
    }

    #[must_use]
    pub fn particle_alpha_speed(&self) -> f64 {
        unsafe { ffi::sk_emitter_node_get_particle_alpha_speed(self.ptr) }
    }

    pub fn set_particle_alpha_speed(&self, speed: f64) {
        unsafe { ffi::sk_emitter_node_set_particle_alpha_speed(self.ptr, speed) };
    }

    #[must_use]
    pub fn field_bitmask(&self) -> u32 {
        unsafe { ffi::sk_emitter_node_get_field_bitmask(self.ptr) }
    }

    pub fn set_field_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_emitter_node_set_field_bitmask(self.ptr, mask) };
    }

    pub fn set_target_node<N: AsNode>(&self, node: Option<&N>) {
        unsafe {
            ffi::sk_emitter_node_set_target_node(
                self.ptr,
                node.map_or(core::ptr::null_mut(), AsNode::as_node_ptr),
            );
        };
    }
}

impl Default for EmitterNode {
    fn default() -> Self {
        Self::new().expect("SKEmitterNode creation failed")
    }
}
