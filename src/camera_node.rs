use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(CameraNode);

impl AsNode for CameraNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl CameraNode {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_camera_node_new()) }
    }

    #[must_use]
    pub fn contains_node<N: AsNode>(&self, node: &N) -> bool {
        unsafe { ffi::sk_camera_node_contains_node(self.ptr, node.as_node_ptr()) }
    }

    #[must_use]
    pub fn contained_node_count(&self) -> usize {
        unsafe { ffi::sk_camera_node_get_contained_node_count(self.ptr) }
    }
}
