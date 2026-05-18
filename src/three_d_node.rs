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
    /// Wraps `SK3DNode`.
    #[must_use]
    pub fn with_viewport_size(size: CGSize) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_3d_node_new_with_viewport_size(
                size.width,
                size.height,
            ))
        }
    }

    /// Wraps `SK3DNode`.
    #[must_use]
    pub fn viewport_size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_3d_node_get_viewport_size_w(self.ptr) },
            unsafe { ffi::sk_3d_node_get_viewport_size_h(self.ptr) },
        )
    }

    /// Sets a property exposed by `SK3DNode`.
    pub fn set_viewport_size(&self, size: CGSize) {
        unsafe { ffi::sk_3d_node_set_viewport_size(self.ptr, size.width, size.height) };
    }

    /// Sets a property exposed by `SK3DNode`.
    pub fn set_empty_scene(&self) {
        unsafe { ffi::sk_3d_node_set_empty_scene(self.ptr) };
    }

    /// Returns a property exposed by `SK3DNode`.
    #[must_use]
    pub fn has_scene(&self) -> bool {
        unsafe { ffi::sk_3d_node_has_scene(self.ptr) }
    }

    /// Wraps `SK3DNode`.
    #[must_use]
    pub fn scene_time(&self) -> f64 {
        unsafe { ffi::sk_3d_node_get_scene_time(self.ptr) }
    }

    /// Sets a property exposed by `SK3DNode`.
    pub fn set_scene_time(&self, time: f64) {
        unsafe { ffi::sk_3d_node_set_scene_time(self.ptr, time) };
    }

    /// Returns a property exposed by `SK3DNode`.
    #[must_use]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::sk_3d_node_get_playing(self.ptr) }
    }

    /// Sets a property exposed by `SK3DNode`.
    pub fn set_playing(&self, playing: bool) {
        unsafe { ffi::sk_3d_node_set_playing(self.ptr, playing) };
    }

    /// Returns a property exposed by `SK3DNode`.
    #[must_use]
    pub fn loops(&self) -> bool {
        unsafe { ffi::sk_3d_node_get_loops(self.ptr) }
    }

    /// Sets a property exposed by `SK3DNode`.
    pub fn set_loops(&self, loops: bool) {
        unsafe { ffi::sk_3d_node_set_loops(self.ptr, loops) };
    }

    /// Wraps `SK3DNode`.
    #[must_use]
    pub fn autoenables_default_lighting(&self) -> bool {
        unsafe { ffi::sk_3d_node_get_autoenables_default_lighting(self.ptr) }
    }

    /// Sets a property exposed by `SK3DNode`.
    pub fn set_autoenables_default_lighting(&self, enabled: bool) {
        unsafe { ffi::sk_3d_node_set_autoenables_default_lighting(self.ptr, enabled) };
    }
}
