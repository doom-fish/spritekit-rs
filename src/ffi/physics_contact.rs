use core::ffi::c_void;

extern "C" {
    pub fn sk_physics_contact_get_body_a(contact: *mut c_void) -> *mut c_void;
    pub fn sk_physics_contact_get_body_b(contact: *mut c_void) -> *mut c_void;
    pub fn sk_physics_contact_get_contact_point_x(contact: *mut c_void) -> f64;
    pub fn sk_physics_contact_get_contact_point_y(contact: *mut c_void) -> f64;
    pub fn sk_physics_contact_get_contact_normal_dx(contact: *mut c_void) -> f64;
    pub fn sk_physics_contact_get_contact_normal_dy(contact: *mut c_void) -> f64;
    pub fn sk_physics_contact_get_collision_impulse(contact: *mut c_void) -> f64;

    pub fn sk_physics_contact_delegate_new(
        context: *mut c_void,
        did_begin: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        did_end: Option<extern "C" fn(*mut c_void, *mut c_void)>,
        release_context: Option<extern "C" fn(*mut c_void)>,
    ) -> *mut c_void;
    pub fn sk_physics_world_set_contact_delegate(world: *mut c_void, delegate: *mut c_void);
    pub fn sk_physics_world_has_contact_delegate(world: *mut c_void) -> bool;
}
