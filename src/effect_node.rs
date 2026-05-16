use crate::ffi;
use crate::node::AsNode;
use crate::physics::BlendMode;
use crate::private::handle_type;

handle_type!(EffectNode);

impl AsNode for EffectNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl EffectNode {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_effect_node_new()) }
    }

    #[must_use]
    pub fn should_enable_effects(&self) -> bool {
        unsafe { ffi::sk_effect_node_get_should_enable_effects(self.ptr) }
    }

    pub fn set_should_enable_effects(&self, enable: bool) {
        unsafe { ffi::sk_effect_node_set_should_enable_effects(self.ptr, enable) };
    }

    #[must_use]
    pub fn should_rasterize(&self) -> bool {
        unsafe { ffi::sk_effect_node_get_should_rasterize(self.ptr) }
    }

    pub fn set_should_rasterize(&self, rasterize: bool) {
        unsafe { ffi::sk_effect_node_set_should_rasterize(self.ptr, rasterize) };
    }

    #[must_use]
    pub fn blend_mode(&self) -> BlendMode {
        BlendMode::from_raw(unsafe { ffi::sk_effect_node_get_blend_mode(self.ptr) })
    }

    pub fn set_blend_mode(&self, mode: BlendMode) {
        unsafe { ffi::sk_effect_node_set_blend_mode(self.ptr, mode as i32) };
    }
}

impl Default for EffectNode {
    fn default() -> Self {
        Self::new().expect("SKEffectNode creation failed")
    }
}
