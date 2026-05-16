use core::ffi::c_void;

extern "C" {
    pub fn sk_3d_node_new_with_viewport_size(width: f64, height: f64) -> *mut c_void;
    pub fn sk_3d_node_get_viewport_size_w(node: *mut c_void) -> f64;
    pub fn sk_3d_node_get_viewport_size_h(node: *mut c_void) -> f64;
    pub fn sk_3d_node_set_viewport_size(node: *mut c_void, width: f64, height: f64);
    pub fn sk_3d_node_set_empty_scene(node: *mut c_void);
    pub fn sk_3d_node_has_scene(node: *mut c_void) -> bool;
    pub fn sk_3d_node_get_scene_time(node: *mut c_void) -> f64;
    pub fn sk_3d_node_set_scene_time(node: *mut c_void, time: f64);
    pub fn sk_3d_node_get_playing(node: *mut c_void) -> bool;
    pub fn sk_3d_node_set_playing(node: *mut c_void, playing: bool);
    pub fn sk_3d_node_get_loops(node: *mut c_void) -> bool;
    pub fn sk_3d_node_set_loops(node: *mut c_void, loops: bool);
    pub fn sk_3d_node_get_autoenables_default_lighting(node: *mut c_void) -> bool;
    pub fn sk_3d_node_set_autoenables_default_lighting(node: *mut c_void, enabled: bool);
}
