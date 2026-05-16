use apple_cf::cg::CGSize;

use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(ThreeDNode);

impl AsNode for ThreeDNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl ThreeDNode {
    #[must_use]
    pub fn with_viewport_size(size: CGSize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_3d_node_new_with_viewport_size(size.width, size.height)) }
    }

    #[must_use]
    pub fn viewport_size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_3d_node_get_viewport_size_w(self.ptr) },
            unsafe { ffi::sk_3d_node_get_viewport_size_h(self.ptr) },
        )
    }

    pub fn set_viewport_size(&self, size: CGSize) {
        unsafe { ffi::sk_3d_node_set_viewport_size(self.ptr, size.width, size.height) };
    }

    pub fn set_empty_scene(&self) {
        unsafe { ffi::sk_3d_node_set_empty_scene(self.ptr) };
    }

    #[must_use]
    pub fn has_scene(&self) -> bool {
        unsafe { ffi::sk_3d_node_has_scene(self.ptr) }
    }

    #[must_use]
    pub fn scene_time(&self) -> f64 {
        unsafe { ffi::sk_3d_node_get_scene_time(self.ptr) }
    }

    pub fn set_scene_time(&self, time: f64) {
        unsafe { ffi::sk_3d_node_set_scene_time(self.ptr, time) };
    }

    #[must_use]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::sk_3d_node_get_playing(self.ptr) }
    }

    pub fn set_playing(&self, playing: bool) {
        unsafe { ffi::sk_3d_node_set_playing(self.ptr, playing) };
    }

    #[must_use]
    pub fn loops(&self) -> bool {
        unsafe { ffi::sk_3d_node_get_loops(self.ptr) }
    }

    pub fn set_loops(&self, loops: bool) {
        unsafe { ffi::sk_3d_node_set_loops(self.ptr, loops) };
    }

    #[must_use]
    pub fn autoenables_default_lighting(&self) -> bool {
        unsafe { ffi::sk_3d_node_get_autoenables_default_lighting(self.ptr) }
    }

    pub fn set_autoenables_default_lighting(&self, enabled: bool) {
        unsafe { ffi::sk_3d_node_set_autoenables_default_lighting(self.ptr, enabled) };
    }
}
