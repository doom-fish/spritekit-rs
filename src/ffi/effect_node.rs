use core::ffi::c_void;

extern "C" {
    /// Wraps `SKEffectNode`.
    pub fn sk_effect_node_new() -> *mut c_void;
    /// Wraps `SKEffectNode`.
    pub fn sk_effect_node_get_should_enable_effects(node: *mut c_void) -> bool;
    /// Wraps `SKEffectNode`.
    pub fn sk_effect_node_set_should_enable_effects(node: *mut c_void, enable: bool);
    /// Wraps `SKEffectNode`.
    pub fn sk_effect_node_get_should_rasterize(node: *mut c_void) -> bool;
    /// Wraps `SKEffectNode`.
    pub fn sk_effect_node_set_should_rasterize(node: *mut c_void, rasterize: bool);
    /// Wraps `SKEffectNode`.
    pub fn sk_effect_node_get_blend_mode(node: *mut c_void) -> i32;
    /// Wraps `SKEffectNode`.
    pub fn sk_effect_node_set_blend_mode(node: *mut c_void, mode: i32);
}
