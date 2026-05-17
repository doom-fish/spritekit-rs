use apple_cf::cg::{CGPoint, CGRect, CGVector};

use crate::ffi;
use crate::physics_body::PhysicsBody;
use crate::physics_contact::PhysicsContactDelegate;
use crate::private::handle_type;

handle_type!(PhysicsWorld);

impl PhysicsWorld {
    #[must_use]
    pub fn gravity(&self) -> CGVector {
        CGVector::new(
            unsafe { ffi::sk_physics_world_get_gravity_dx(self.ptr) },
            unsafe { ffi::sk_physics_world_get_gravity_dy(self.ptr) },
        )
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

    #[must_use]
    pub fn body_at_point(&self, point: CGPoint) -> Option<PhysicsBody> {
        unsafe {
            PhysicsBody::from_raw(ffi::sk_physics_world_body_at_point(
                self.ptr, point.x, point.y,
            ))
        }
    }

    #[must_use]
    pub fn body_in_rect(&self, rect: CGRect) -> Option<PhysicsBody> {
        unsafe {
            PhysicsBody::from_raw(ffi::sk_physics_world_body_in_rect(
                self.ptr,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            ))
        }
    }

    #[must_use]
    pub fn body_along_ray(&self, start: CGPoint, end: CGPoint) -> Option<PhysicsBody> {
        unsafe {
            PhysicsBody::from_raw(ffi::sk_physics_world_body_along_ray(
                self.ptr, start.x, start.y, end.x, end.y,
            ))
        }
    }

    pub fn set_contact_delegate(&self, delegate: Option<&PhysicsContactDelegate>) {
        unsafe {
            ffi::sk_physics_world_set_contact_delegate(
                self.ptr,
                delegate.map_or(core::ptr::null_mut(), PhysicsContactDelegate::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn has_contact_delegate(&self) -> bool {
        unsafe { ffi::sk_physics_world_has_contact_delegate(self.ptr) }
    }
}
