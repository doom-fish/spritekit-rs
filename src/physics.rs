use apple_cf::cg::{CGRect, CGVector};

use crate::ffi;
use crate::private::handle_type;
use crate::texture::Texture;

handle_type!(PhysicsBody);
handle_type!(PhysicsWorld);

/// `SKBlendMode` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum BlendMode {
    #[default]
    Alpha = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    MultiplyX2 = 4,
    Screen = 5,
    Replace = 6,
    MultiplyAlpha = 7,
}

impl BlendMode {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Subtract,
            3 => Self::Multiply,
            4 => Self::MultiplyX2,
            5 => Self::Screen,
            6 => Self::Replace,
            7 => Self::MultiplyAlpha,
            _ => Self::Alpha,
        }
    }
}

impl PhysicsBody {
    /// Dynamic body with a circular collision shape.
    #[must_use]
    pub fn circle(radius: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_physics_body_circle(radius)) }
    }

    /// Dynamic body with a rectangular collision shape.
    #[must_use]
    pub fn rect(width: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_physics_body_rect(width, height)) }
    }

    /// Static edge-loop body enclosing the given rectangle.
    #[must_use]
    pub fn edge_loop_rect(rect: CGRect) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_body_edge_loop_rect(
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            ))
        }
    }

    /// Body shaped from a texture's alpha channel.
    #[must_use]
    pub fn from_texture(texture: &Texture, width: f64, height: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_body_texture(
                texture.as_ptr(),
                width,
                height,
            ))
        }
    }

    // --- simulation flags ---

    #[must_use]
    pub fn is_dynamic(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_dynamic(self.ptr) }
    }

    pub fn set_dynamic(&self, dynamic: bool) {
        unsafe { ffi::sk_physics_body_set_dynamic(self.ptr, dynamic) };
    }

    #[must_use]
    pub fn allows_rotation(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_allows_rotation(self.ptr) }
    }

    pub fn set_allows_rotation(&self, allows: bool) {
        unsafe { ffi::sk_physics_body_set_allows_rotation(self.ptr, allows) };
    }

    #[must_use]
    pub fn uses_precise_collision_detection(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_precise_collision(self.ptr) }
    }

    pub fn set_uses_precise_collision_detection(&self, precise: bool) {
        unsafe { ffi::sk_physics_body_set_precise_collision(self.ptr, precise) };
    }

    #[must_use]
    pub fn is_pinned(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_pinned(self.ptr) }
    }

    pub fn set_pinned(&self, pinned: bool) {
        unsafe { ffi::sk_physics_body_set_pinned(self.ptr, pinned) };
    }

    // --- material ---

    #[must_use]
    pub fn friction(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_friction(self.ptr) }
    }

    pub fn set_friction(&self, friction: f64) {
        unsafe { ffi::sk_physics_body_set_friction(self.ptr, friction) };
    }

    #[must_use]
    pub fn restitution(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_restitution(self.ptr) }
    }

    pub fn set_restitution(&self, restitution: f64) {
        unsafe { ffi::sk_physics_body_set_restitution(self.ptr, restitution) };
    }

    #[must_use]
    pub fn linear_damping(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_linear_damping(self.ptr) }
    }

    pub fn set_linear_damping(&self, damping: f64) {
        unsafe { ffi::sk_physics_body_set_linear_damping(self.ptr, damping) };
    }

    #[must_use]
    pub fn angular_damping(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_angular_damping(self.ptr) }
    }

    pub fn set_angular_damping(&self, damping: f64) {
        unsafe { ffi::sk_physics_body_set_angular_damping(self.ptr, damping) };
    }

    #[must_use]
    pub fn density(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_density(self.ptr) }
    }

    pub fn set_density(&self, density: f64) {
        unsafe { ffi::sk_physics_body_set_density(self.ptr, density) };
    }

    #[must_use]
    pub fn mass(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_mass(self.ptr) }
    }

    pub fn set_mass(&self, mass: f64) {
        unsafe { ffi::sk_physics_body_set_mass(self.ptr, mass) };
    }

    #[must_use]
    pub fn is_affected_by_gravity(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_gravity_scale(self.ptr) > 0.0 }
    }

    pub fn set_affected_by_gravity(&self, affected: bool) {
        unsafe { ffi::sk_physics_body_set_affected_by_gravity(self.ptr, affected) };
    }

    // --- bitmasks ---

    #[must_use]
    pub fn category_bitmask(&self) -> u32 {
        unsafe { ffi::sk_physics_body_get_category_bitmask(self.ptr) }
    }

    pub fn set_category_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_physics_body_set_category_bitmask(self.ptr, mask) };
    }

    #[must_use]
    pub fn contact_test_bitmask(&self) -> u32 {
        unsafe { ffi::sk_physics_body_get_contact_test_bitmask(self.ptr) }
    }

    pub fn set_contact_test_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_physics_body_set_contact_test_bitmask(self.ptr, mask) };
    }

    #[must_use]
    pub fn collision_bitmask(&self) -> u32 {
        unsafe { ffi::sk_physics_body_get_collision_bitmask(self.ptr) }
    }

    pub fn set_collision_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_physics_body_set_collision_bitmask(self.ptr, mask) };
    }

    // --- velocity / forces ---

    #[must_use]
    pub fn velocity(&self) -> CGVector {
        let dx = unsafe { ffi::sk_physics_body_get_velocity_dx(self.ptr) };
        let dy = unsafe { ffi::sk_physics_body_get_velocity_dy(self.ptr) };
        CGVector::new(dx, dy)
    }

    pub fn set_velocity(&self, velocity: CGVector) {
        unsafe { ffi::sk_physics_body_set_velocity(self.ptr, velocity.dx, velocity.dy) };
    }

    pub fn apply_force(&self, force: CGVector) {
        unsafe { ffi::sk_physics_body_apply_force(self.ptr, force.dx, force.dy) };
    }

    pub fn apply_impulse(&self, impulse: CGVector) {
        unsafe { ffi::sk_physics_body_apply_impulse(self.ptr, impulse.dx, impulse.dy) };
    }
}

impl PhysicsWorld {
    #[must_use]
    pub fn gravity(&self) -> CGVector {
        let dx = unsafe { ffi::sk_physics_world_get_gravity_dx(self.ptr) };
        let dy = unsafe { ffi::sk_physics_world_get_gravity_dy(self.ptr) };
        CGVector::new(dx, dy)
    }

    pub fn set_gravity(&self, gravity: CGVector) {
        unsafe { ffi::sk_physics_world_set_gravity(self.ptr, gravity.dx, gravity.dy) };
    }

    #[must_use]
    pub fn speed(&self) -> f64 {
        unsafe { ffi::sk_physics_world_get_speed(self.ptr) }
    }

    pub fn set_speed(&self, speed: f64) {
        unsafe { ffi::sk_physics_world_set_speed(self.ptr, speed) };
    }
}
