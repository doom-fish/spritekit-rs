use core::ffi::c_void;

extern "C" {
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_new_with_frame(x: f64, y: f64, width: f64, height: f64) -> *mut c_void;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_paused(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_paused(view: *mut c_void, paused: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_shows_fps(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_shows_fps(view: *mut c_void, shows: bool);
    /// Returns a property exposed by `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_shows_draw_count(view: *mut c_void) -> bool;
    /// Returns a property exposed by `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_shows_draw_count(view: *mut c_void, shows: bool);
    /// Returns a property exposed by `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_shows_node_count(view: *mut c_void) -> bool;
    /// Returns a property exposed by `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_shows_node_count(view: *mut c_void, shows: bool);
    /// Returns a property exposed by `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_shows_quad_count(view: *mut c_void) -> bool;
    /// Returns a property exposed by `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_shows_quad_count(view: *mut c_void, shows: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_shows_physics(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_shows_physics(view: *mut c_void, shows: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_shows_fields(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_shows_fields(view: *mut c_void, shows: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_asynchronous(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_asynchronous(view: *mut c_void, asynchronous: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_allows_transparency(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_allows_transparency(view: *mut c_void, allows: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_ignores_sibling_order(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_ignores_sibling_order(view: *mut c_void, ignores: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_should_cull_non_visible_nodes(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_should_cull_non_visible_nodes(view: *mut c_void, should_cull: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_preferred_frames_per_second(view: *mut c_void) -> isize;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_preferred_frames_per_second(view: *mut c_void, fps: isize);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_disable_depth_stencil_buffer(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_disable_depth_stencil_buffer(view: *mut c_void, disable: bool);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_delegate_new(
        context: *mut c_void,
        should_render: Option<extern "C" fn(*mut c_void, f64) -> bool>,
        release_context: Option<extern "C" fn(*mut c_void)>,
    ) -> *mut c_void;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_set_delegate(view: *mut c_void, delegate: *mut c_void);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_has_delegate(view: *mut c_void) -> bool;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_present_scene(view: *mut c_void, scene: *mut c_void);
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_present_scene_with_transition(
        view: *mut c_void,
        scene: *mut c_void,
        transition: *mut c_void,
    );
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_get_scene(view: *mut c_void) -> *mut c_void;
    /// Wraps `SKView` and `SKViewDelegate`.
    pub fn sk_view_texture_from_node(view: *mut c_void, node: *mut c_void) -> *mut c_void;
}
