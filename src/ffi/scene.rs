use core::ffi::c_void;

extern "C" {
    pub fn sk_scene_new_with_size(width: f64, height: f64) -> *mut c_void;
    pub fn sk_scene_get_size_w(scene: *mut c_void) -> f64;
    pub fn sk_scene_get_size_h(scene: *mut c_void) -> f64;
    pub fn sk_scene_set_size(scene: *mut c_void, width: f64, height: f64);
    pub fn sk_scene_get_scale_mode(scene: *mut c_void) -> i32;
    pub fn sk_scene_set_scale_mode(scene: *mut c_void, mode: i32);
    pub fn sk_scene_set_background_color(scene: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn sk_scene_get_camera(scene: *mut c_void) -> *mut c_void;
    pub fn sk_scene_set_camera(scene: *mut c_void, camera: *mut c_void);
    pub fn sk_scene_delegate_new(
        context: *mut c_void,
        update: Option<extern "C" fn(*mut c_void, f64)>,
        release_context: Option<extern "C" fn(*mut c_void)>,
    ) -> *mut c_void;
    pub fn sk_scene_set_delegate(scene: *mut c_void, delegate: *mut c_void);
    pub fn sk_scene_has_delegate(scene: *mut c_void) -> bool;
    pub fn sk_scene_get_anchor_x(scene: *mut c_void) -> f64;
    pub fn sk_scene_get_anchor_y(scene: *mut c_void) -> f64;
    pub fn sk_scene_set_anchor_point(scene: *mut c_void, x: f64, y: f64);
    pub fn sk_scene_physics_world(scene: *mut c_void) -> *mut c_void;
    pub fn sk_scene_get_view(scene: *mut c_void) -> *mut c_void;
    pub fn sk_scene_convert_point_from_view_x(scene: *mut c_void, x: f64, y: f64) -> f64;
    pub fn sk_scene_convert_point_from_view_y(scene: *mut c_void, x: f64, y: f64) -> f64;
    pub fn sk_scene_convert_point_to_view_x(scene: *mut c_void, x: f64, y: f64) -> f64;
    pub fn sk_scene_convert_point_to_view_y(scene: *mut c_void, x: f64, y: f64) -> f64;
}
