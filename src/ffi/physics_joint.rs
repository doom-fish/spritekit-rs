use core::ffi::c_void;

extern "C" {
    pub fn sk_physics_joint_get_reaction_force_dx(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_get_reaction_force_dy(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_get_reaction_torque(joint: *mut c_void) -> f64;

    pub fn sk_physics_joint_pin_new(body_a: *mut c_void, body_b: *mut c_void, x: f64, y: f64) -> *mut c_void;
    pub fn sk_physics_joint_pin_get_should_enable_limits(joint: *mut c_void) -> bool;
    pub fn sk_physics_joint_pin_set_should_enable_limits(joint: *mut c_void, enabled: bool);
    pub fn sk_physics_joint_pin_get_lower_angle_limit(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_pin_set_lower_angle_limit(joint: *mut c_void, value: f64);
    pub fn sk_physics_joint_pin_get_upper_angle_limit(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_pin_set_upper_angle_limit(joint: *mut c_void, value: f64);
    pub fn sk_physics_joint_pin_get_friction_torque(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_pin_set_friction_torque(joint: *mut c_void, value: f64);
    pub fn sk_physics_joint_pin_get_rotation_speed(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_pin_set_rotation_speed(joint: *mut c_void, value: f64);

    pub fn sk_physics_joint_spring_new(body_a: *mut c_void, body_b: *mut c_void, anchor_a_x: f64, anchor_a_y: f64, anchor_b_x: f64, anchor_b_y: f64) -> *mut c_void;
    pub fn sk_physics_joint_spring_get_damping(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_spring_set_damping(joint: *mut c_void, value: f64);
    pub fn sk_physics_joint_spring_get_frequency(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_spring_set_frequency(joint: *mut c_void, value: f64);

    pub fn sk_physics_joint_fixed_new(body_a: *mut c_void, body_b: *mut c_void, x: f64, y: f64) -> *mut c_void;

    pub fn sk_physics_joint_sliding_new(body_a: *mut c_void, body_b: *mut c_void, anchor_x: f64, anchor_y: f64, axis_dx: f64, axis_dy: f64) -> *mut c_void;
    pub fn sk_physics_joint_sliding_get_should_enable_limits(joint: *mut c_void) -> bool;
    pub fn sk_physics_joint_sliding_set_should_enable_limits(joint: *mut c_void, enabled: bool);
    pub fn sk_physics_joint_sliding_get_lower_distance_limit(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_sliding_set_lower_distance_limit(joint: *mut c_void, value: f64);
    pub fn sk_physics_joint_sliding_get_upper_distance_limit(joint: *mut c_void) -> f64;
    pub fn sk_physics_joint_sliding_set_upper_distance_limit(joint: *mut c_void, value: f64);
}
