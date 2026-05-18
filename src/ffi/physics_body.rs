use core::ffi::c_void;

extern "C" {
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_circle(radius: f64) -> *mut c_void;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_circle_center(radius: f64, x: f64, y: f64) -> *mut c_void;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_rect(width: f64, height: f64) -> *mut c_void;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_rect_center(width: f64, height: f64, x: f64, y: f64) -> *mut c_void;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_edge_loop_rect(x: f64, y: f64, width: f64, height: f64) -> *mut c_void;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_texture(texture: *mut c_void, size_w: f64, size_h: f64) -> *mut c_void;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_compound(bodies: *mut c_void, count: usize) -> *mut c_void;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_dynamic(body: *mut c_void) -> bool;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_dynamic(body: *mut c_void, dynamic: bool);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_allows_rotation(body: *mut c_void) -> bool;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_allows_rotation(body: *mut c_void, allows: bool);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_precise_collision(body: *mut c_void) -> bool;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_precise_collision(body: *mut c_void, precise: bool);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_pinned(body: *mut c_void) -> bool;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_pinned(body: *mut c_void, pinned: bool);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_resting(body: *mut c_void) -> bool;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_resting(body: *mut c_void, resting: bool);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_friction(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_friction(body: *mut c_void, friction: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_charge(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_charge(body: *mut c_void, charge: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_restitution(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_restitution(body: *mut c_void, restitution: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_linear_damping(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_linear_damping(body: *mut c_void, damping: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_angular_damping(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_angular_damping(body: *mut c_void, damping: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_density(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_density(body: *mut c_void, density: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_mass(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_mass(body: *mut c_void, mass: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_area(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_affected_by_gravity(body: *mut c_void) -> bool;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_affected_by_gravity(body: *mut c_void, affected: bool);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_field_bitmask(body: *mut c_void) -> u32;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_field_bitmask(body: *mut c_void, mask: u32);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_category_bitmask(body: *mut c_void) -> u32;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_category_bitmask(body: *mut c_void, mask: u32);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_contact_test_bitmask(body: *mut c_void) -> u32;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_contact_test_bitmask(body: *mut c_void, mask: u32);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_collision_bitmask(body: *mut c_void) -> u32;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_collision_bitmask(body: *mut c_void, mask: u32);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_velocity_dx(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_velocity_dy(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_velocity(body: *mut c_void, dx: f64, dy: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_get_angular_velocity(body: *mut c_void) -> f64;
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_set_angular_velocity(body: *mut c_void, velocity: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_apply_force(body: *mut c_void, dx: f64, dy: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_apply_force_at_point(
        body: *mut c_void,
        force_dx: f64,
        force_dy: f64,
        point_x: f64,
        point_y: f64,
    );
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_apply_torque(body: *mut c_void, torque: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_apply_impulse(body: *mut c_void, dx: f64, dy: f64);
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_apply_impulse_at_point(
        body: *mut c_void,
        impulse_dx: f64,
        impulse_dy: f64,
        point_x: f64,
        point_y: f64,
    );
    /// Wraps `SKPhysicsBody`.
    pub fn sk_physics_body_apply_angular_impulse(body: *mut c_void, impulse: f64);
    /// Returns a property exposed by `SKPhysicsBody`.
    pub fn sk_physics_body_all_contacted_bodies_count(body: *mut c_void) -> usize;
}
