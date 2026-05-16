use core::ffi::c_void;

extern "C" {
    pub fn sk_video_node_new() -> *mut c_void;
    pub fn sk_video_node_play(node: *mut c_void);
    pub fn sk_video_node_pause(node: *mut c_void);
    pub fn sk_video_node_get_size_w(node: *mut c_void) -> f64;
    pub fn sk_video_node_get_size_h(node: *mut c_void) -> f64;
    pub fn sk_video_node_set_size(node: *mut c_void, width: f64, height: f64);
    pub fn sk_video_node_get_anchor_x(node: *mut c_void) -> f64;
    pub fn sk_video_node_get_anchor_y(node: *mut c_void) -> f64;
    pub fn sk_video_node_set_anchor_point(node: *mut c_void, x: f64, y: f64);
}
