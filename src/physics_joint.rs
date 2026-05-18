use apple_cf::cg::{CGPoint, CGVector};

use crate::ffi;
use crate::physics_body::PhysicsBody;
use crate::private::handle_type;

/// Trait for `SKPhysicsJoint`.
pub trait AsPhysicsJoint {
    #[doc(hidden)]
    fn as_joint_ptr(&self) -> *mut core::ffi::c_void;
}

/// Trait for `SKPhysicsJoint`.
pub trait PhysicsJointExt: AsPhysicsJoint {
    #[must_use]
    fn reaction_force(&self) -> CGVector {
        CGVector::new(
            unsafe { ffi::sk_physics_joint_get_reaction_force_dx(self.as_joint_ptr()) },
            unsafe { ffi::sk_physics_joint_get_reaction_force_dy(self.as_joint_ptr()) },
        )
    }

    #[must_use]
    fn reaction_torque(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_get_reaction_torque(self.as_joint_ptr()) }
    }
}

impl<T: AsPhysicsJoint> PhysicsJointExt for T {}

handle_type!(PhysicsJointPin);
handle_type!(PhysicsJointSpring);
handle_type!(PhysicsJointFixed);
handle_type!(PhysicsJointSliding);
handle_type!(PhysicsJointLimit);

impl AsPhysicsJoint for PhysicsJointPin {
    fn as_joint_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl AsPhysicsJoint for PhysicsJointSpring {
    fn as_joint_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl AsPhysicsJoint for PhysicsJointFixed {
    fn as_joint_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl AsPhysicsJoint for PhysicsJointSliding {
    fn as_joint_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl AsPhysicsJoint for PhysicsJointLimit {
    fn as_joint_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl PhysicsJointPin {
    /// Wraps `SKPhysicsJointPin`.
    #[must_use]
    pub fn new(body_a: &PhysicsBody, body_b: &PhysicsBody, anchor: CGPoint) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_joint_pin_new(
                body_a.as_ptr(),
                body_b.as_ptr(),
                anchor.x,
                anchor.y,
            ))
        }
    }

    /// Returns a property exposed by `SKPhysicsJointPin`.
    #[must_use]
    pub fn should_enable_limits(&self) -> bool {
        unsafe { ffi::sk_physics_joint_pin_get_should_enable_limits(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointPin`.
    pub fn set_should_enable_limits(&self, enabled: bool) {
        unsafe { ffi::sk_physics_joint_pin_set_should_enable_limits(self.ptr, enabled) };
    }

    /// Returns a property exposed by `SKPhysicsJointPin`.
    #[must_use]
    pub fn lower_angle_limit(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_pin_get_lower_angle_limit(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointPin`.
    pub fn set_lower_angle_limit(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_pin_set_lower_angle_limit(self.ptr, value) };
    }

    /// Returns a property exposed by `SKPhysicsJointPin`.
    #[must_use]
    pub fn upper_angle_limit(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_pin_get_upper_angle_limit(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointPin`.
    pub fn set_upper_angle_limit(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_pin_set_upper_angle_limit(self.ptr, value) };
    }

    /// Returns a property exposed by `SKPhysicsJointPin`.
    #[must_use]
    pub fn friction_torque(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_pin_get_friction_torque(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointPin`.
    pub fn set_friction_torque(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_pin_set_friction_torque(self.ptr, value) };
    }

    /// Returns a property exposed by `SKPhysicsJointPin`.
    #[must_use]
    pub fn rotation_speed(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_pin_get_rotation_speed(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointPin`.
    pub fn set_rotation_speed(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_pin_set_rotation_speed(self.ptr, value) };
    }
}

impl PhysicsJointSpring {
    /// Wraps `SKPhysicsJointSpring`.
    #[must_use]
    pub fn new(
        body_a: &PhysicsBody,
        body_b: &PhysicsBody,
        anchor_a: CGPoint,
        anchor_b: CGPoint,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_joint_spring_new(
                body_a.as_ptr(),
                body_b.as_ptr(),
                anchor_a.x,
                anchor_a.y,
                anchor_b.x,
                anchor_b.y,
            ))
        }
    }

    /// Returns a property exposed by `SKPhysicsJointSpring`.
    #[must_use]
    pub fn damping(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_spring_get_damping(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointSpring`.
    pub fn set_damping(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_spring_set_damping(self.ptr, value) };
    }

    /// Returns a property exposed by `SKPhysicsJointSpring`.
    #[must_use]
    pub fn frequency(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_spring_get_frequency(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointSpring`.
    pub fn set_frequency(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_spring_set_frequency(self.ptr, value) };
    }
}

impl PhysicsJointFixed {
    /// Wraps `SKPhysicsJointFixed`.
    #[must_use]
    pub fn new(body_a: &PhysicsBody, body_b: &PhysicsBody, anchor: CGPoint) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_joint_fixed_new(
                body_a.as_ptr(),
                body_b.as_ptr(),
                anchor.x,
                anchor.y,
            ))
        }
    }
}

impl PhysicsJointSliding {
    /// Wraps `SKPhysicsJointSliding`.
    #[must_use]
    pub fn new(
        body_a: &PhysicsBody,
        body_b: &PhysicsBody,
        anchor: CGPoint,
        axis: CGVector,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_joint_sliding_new(
                body_a.as_ptr(),
                body_b.as_ptr(),
                anchor.x,
                anchor.y,
                axis.dx,
                axis.dy,
            ))
        }
    }

    /// Returns a property exposed by `SKPhysicsJointSliding`.
    #[must_use]
    pub fn should_enable_limits(&self) -> bool {
        unsafe { ffi::sk_physics_joint_sliding_get_should_enable_limits(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointSliding`.
    pub fn set_should_enable_limits(&self, enabled: bool) {
        unsafe { ffi::sk_physics_joint_sliding_set_should_enable_limits(self.ptr, enabled) };
    }

    /// Returns a property exposed by `SKPhysicsJointSliding`.
    #[must_use]
    pub fn lower_distance_limit(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_sliding_get_lower_distance_limit(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointSliding`.
    pub fn set_lower_distance_limit(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_sliding_set_lower_distance_limit(self.ptr, value) };
    }

    /// Returns a property exposed by `SKPhysicsJointSliding`.
    #[must_use]
    pub fn upper_distance_limit(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_sliding_get_upper_distance_limit(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointSliding`.
    pub fn set_upper_distance_limit(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_sliding_set_upper_distance_limit(self.ptr, value) };
    }
}

impl PhysicsJointLimit {
    /// Wraps `SKPhysicsJointLimit`.
    #[must_use]
    pub fn new(
        body_a: &PhysicsBody,
        body_b: &PhysicsBody,
        anchor_a: CGPoint,
        anchor_b: CGPoint,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_physics_joint_limit_new(
                body_a.as_ptr(),
                body_b.as_ptr(),
                anchor_a.x,
                anchor_a.y,
                anchor_b.x,
                anchor_b.y,
            ))
        }
    }

    /// Returns a property exposed by `SKPhysicsJointLimit`.
    #[must_use]
    pub fn max_length(&self) -> f64 {
        unsafe { ffi::sk_physics_joint_limit_get_max_length(self.ptr) }
    }

    /// Sets a property exposed by `SKPhysicsJointLimit`.
    pub fn set_max_length(&self, value: f64) {
        unsafe { ffi::sk_physics_joint_limit_set_max_length(self.ptr, value) };
    }
}
