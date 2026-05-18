use core::ffi::c_void;

extern "C" {
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_get_gravity_dx(world: *mut c_void) -> f64;
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_get_gravity_dy(world: *mut c_void) -> f64;
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_set_gravity(world: *mut c_void, dx: f64, dy: f64);
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_get_speed(world: *mut c_void) -> f64;
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_set_speed(world: *mut c_void, speed: f64);
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_body_at_point(world: *mut c_void, x: f64, y: f64) -> *mut c_void;
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_body_in_rect(
        world: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    /// Wraps `SKPhysicsWorld`.
    pub fn sk_physics_world_body_along_ray(
        world: *mut c_void,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
    ) -> *mut c_void;
}
