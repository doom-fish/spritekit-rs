use apple_cf::cg::{CGPoint, CGSize};

use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(VideoNode);

impl AsNode for VideoNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl VideoNode {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_video_node_new()) }
    }

    pub fn play(&self) {
        unsafe { ffi::sk_video_node_play(self.ptr) };
    }

    pub fn pause(&self) {
        unsafe { ffi::sk_video_node_pause(self.ptr) };
    }

    #[must_use]
    pub fn size(&self) -> CGSize {
        CGSize::new(unsafe { ffi::sk_video_node_get_size_w(self.ptr) }, unsafe {
            ffi::sk_video_node_get_size_h(self.ptr)
        })
    }

    pub fn set_size(&self, size: CGSize) {
        unsafe { ffi::sk_video_node_set_size(self.ptr, size.width, size.height) };
    }

    #[must_use]
    pub fn anchor_point(&self) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_video_node_get_anchor_x(self.ptr) },
            unsafe { ffi::sk_video_node_get_anchor_y(self.ptr) },
        )
    }

    pub fn set_anchor_point(&self, anchor: CGPoint) {
        unsafe { ffi::sk_video_node_set_anchor_point(self.ptr, anchor.x, anchor.y) };
    }
}

impl Default for VideoNode {
    fn default() -> Self {
        Self::new().expect("SKVideoNode creation failed")
    }
}
