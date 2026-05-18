use core::ffi::c_void;

use apple_cf::cg::{CGPoint, CGRect, CGSize, CGVector};

use crate::ffi;
use crate::private::handle_type;
use crate::texture::Texture;

handle_type!(PhysicsBody);

impl PhysicsBody {
    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn circle(radius: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_physics_body_circle(radius)) }
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn circle_at(radius: f64, center: CGPoint) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_body_circle_center(
                radius, center.x, center.y,
            ))
        }
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn rect(width: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_physics_body_rect(width, height)) }
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn rect_at(size: CGSize, center: CGPoint) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_body_rect_center(
                size.width,
                size.height,
                center.x,
                center.y,
            ))
        }
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn edge_loop_rect(rect: CGRect) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_body_edge_loop_rect(
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            ))
        }
    }

    /// Wraps `SKPhysicsBody`.
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

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn compound(bodies: &[&Self]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = bodies.iter().map(|body| body.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe { Self::from_raw(ffi::sk_physics_body_compound(raw_ptr, raw.len())) }
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn is_dynamic(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_dynamic(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_dynamic(&self, dynamic: bool) {
        unsafe { ffi::sk_physics_body_set_dynamic(self.ptr, dynamic) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn allows_rotation(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_allows_rotation(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_allows_rotation(&self, allows: bool) {
        unsafe { ffi::sk_physics_body_set_allows_rotation(self.ptr, allows) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn uses_precise_collision_detection(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_precise_collision(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_uses_precise_collision_detection(&self, precise: bool) {
        unsafe { ffi::sk_physics_body_set_precise_collision(self.ptr, precise) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn is_pinned(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_pinned(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_pinned(&self, pinned: bool) {
        unsafe { ffi::sk_physics_body_set_pinned(self.ptr, pinned) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn is_resting(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_resting(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_resting(&self, resting: bool) {
        unsafe { ffi::sk_physics_body_set_resting(self.ptr, resting) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn friction(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_friction(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_friction(&self, friction: f64) {
        unsafe { ffi::sk_physics_body_set_friction(self.ptr, friction) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn charge(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_charge(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_charge(&self, charge: f64) {
        unsafe { ffi::sk_physics_body_set_charge(self.ptr, charge) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn restitution(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_restitution(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_restitution(&self, restitution: f64) {
        unsafe { ffi::sk_physics_body_set_restitution(self.ptr, restitution) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn linear_damping(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_linear_damping(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_linear_damping(&self, damping: f64) {
        unsafe { ffi::sk_physics_body_set_linear_damping(self.ptr, damping) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn angular_damping(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_angular_damping(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_angular_damping(&self, damping: f64) {
        unsafe { ffi::sk_physics_body_set_angular_damping(self.ptr, damping) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn density(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_density(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_density(&self, density: f64) {
        unsafe { ffi::sk_physics_body_set_density(self.ptr, density) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn mass(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_mass(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_mass(&self, mass: f64) {
        unsafe { ffi::sk_physics_body_set_mass(self.ptr, mass) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn area(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_area(self.ptr) }
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn is_affected_by_gravity(&self) -> bool {
        unsafe { ffi::sk_physics_body_get_affected_by_gravity(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_affected_by_gravity(&self, affected: bool) {
        unsafe { ffi::sk_physics_body_set_affected_by_gravity(self.ptr, affected) };
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn field_bitmask(&self) -> u32 {
        unsafe { ffi::sk_physics_body_get_field_bitmask(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_field_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_physics_body_set_field_bitmask(self.ptr, mask) };
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn category_bitmask(&self) -> u32 {
        unsafe { ffi::sk_physics_body_get_category_bitmask(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_category_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_physics_body_set_category_bitmask(self.ptr, mask) };
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn contact_test_bitmask(&self) -> u32 {
        unsafe { ffi::sk_physics_body_get_contact_test_bitmask(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_contact_test_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_physics_body_set_contact_test_bitmask(self.ptr, mask) };
    }

    /// Wraps `SKPhysicsBody`.
    #[must_use]
    pub fn collision_bitmask(&self) -> u32 {
        unsafe { ffi::sk_physics_body_get_collision_bitmask(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_collision_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_physics_body_set_collision_bitmask(self.ptr, mask) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn velocity(&self) -> CGVector {
        CGVector::new(
            unsafe { ffi::sk_physics_body_get_velocity_dx(self.ptr) },
            unsafe { ffi::sk_physics_body_get_velocity_dy(self.ptr) },
        )
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_velocity(&self, velocity: CGVector) {
        unsafe { ffi::sk_physics_body_set_velocity(self.ptr, velocity.dx, velocity.dy) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn angular_velocity(&self) -> f64 {
        unsafe { ffi::sk_physics_body_get_angular_velocity(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsBody`.
    pub fn set_angular_velocity(&self, velocity: f64) {
        unsafe { ffi::sk_physics_body_set_angular_velocity(self.ptr, velocity) };
    }

    /// Wraps `SKPhysicsBody`.
    pub fn apply_force(&self, force: CGVector) {
        unsafe { ffi::sk_physics_body_apply_force(self.ptr, force.dx, force.dy) };
    }

    /// Wraps `SKPhysicsBody`.
    pub fn apply_force_at_point(&self, force: CGVector, point: CGPoint) {
        unsafe {
            ffi::sk_physics_body_apply_force_at_point(
                self.ptr, force.dx, force.dy, point.x, point.y,
            );
        };
    }

    /// Wraps `SKPhysicsBody`.
    pub fn apply_torque(&self, torque: f64) {
        unsafe { ffi::sk_physics_body_apply_torque(self.ptr, torque) };
    }

    /// Wraps `SKPhysicsBody`.
    pub fn apply_impulse(&self, impulse: CGVector) {
        unsafe { ffi::sk_physics_body_apply_impulse(self.ptr, impulse.dx, impulse.dy) };
    }

    /// Wraps `SKPhysicsBody`.
    pub fn apply_impulse_at_point(&self, impulse: CGVector, point: CGPoint) {
        unsafe {
            ffi::sk_physics_body_apply_impulse_at_point(
                self.ptr, impulse.dx, impulse.dy, point.x, point.y,
            );
        };
    }

    /// Wraps `SKPhysicsBody`.
    pub fn apply_angular_impulse(&self, impulse: f64) {
        unsafe { ffi::sk_physics_body_apply_angular_impulse(self.ptr, impulse) };
    }

    /// Returns a property exposed by `SKPhysicsBody`.
    #[must_use]
    pub fn all_contacted_bodies_count(&self) -> usize {
        unsafe { ffi::sk_physics_body_all_contacted_bodies_count(self.ptr) }
    }
}
