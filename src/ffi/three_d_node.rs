use core::ffi::c_void;

extern "C" {
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_new_with_viewport_size(width: f64, height: f64) -> *mut c_void;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_get_viewport_size_w(node: *mut c_void) -> f64;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_get_viewport_size_h(node: *mut c_void) -> f64;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_set_viewport_size(node: *mut c_void, width: f64, height: f64);
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_set_empty_scene(node: *mut c_void);
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_has_scene(node: *mut c_void) -> bool;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_get_scene_time(node: *mut c_void) -> f64;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_set_scene_time(node: *mut c_void, time: f64);
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_get_playing(node: *mut c_void) -> bool;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_set_playing(node: *mut c_void, playing: bool);
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_get_loops(node: *mut c_void) -> bool;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_set_loops(node: *mut c_void, loops: bool);
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_get_autoenables_default_lighting(node: *mut c_void) -> bool;
    /// Wraps `SK3DNode`.
    pub fn sk_3d_node_set_autoenables_default_lighting(node: *mut c_void, enabled: bool);
}
