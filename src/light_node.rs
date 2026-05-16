use crate::color::Color;
use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(LightNode);

impl AsNode for LightNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl LightNode {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_light_node_new()) }
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::sk_light_node_get_enabled(self.ptr) }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe { ffi::sk_light_node_set_enabled(self.ptr, enabled) };
    }

    pub fn set_light_color(&self, color: Color) {
        unsafe { ffi::sk_light_node_set_light_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    pub fn set_ambient_color(&self, color: Color) {
        unsafe { ffi::sk_light_node_set_ambient_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    pub fn set_shadow_color(&self, color: Color) {
        unsafe { ffi::sk_light_node_set_shadow_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    #[must_use]
    pub fn falloff(&self) -> f64 {
        unsafe { ffi::sk_light_node_get_falloff(self.ptr) }
    }

    pub fn set_falloff(&self, falloff: f64) {
        unsafe { ffi::sk_light_node_set_falloff(self.ptr, falloff) };
    }

    #[must_use]
    pub fn category_bitmask(&self) -> u32 {
        unsafe { ffi::sk_light_node_get_category_bitmask(self.ptr) }
    }

    pub fn set_category_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_light_node_set_category_bitmask(self.ptr, mask) };
    }
}

impl Default for LightNode {
    fn default() -> Self {
        Self::new().expect("SKLightNode creation failed")
    }
}
