use crate::ffi;
use crate::node::{AsNode, Node};
use crate::private::handle_type;

handle_type!(CropNode);

impl AsNode for CropNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl CropNode {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_crop_node_new()) }
    }

    #[must_use]
    pub fn mask_node(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::sk_crop_node_get_mask_node(self.ptr)) }
    }

    pub fn set_mask_node<N: AsNode>(&self, node: Option<&N>) {
        unsafe {
            ffi::sk_crop_node_set_mask_node(
                self.ptr,
                node.map_or(core::ptr::null_mut(), AsNode::as_node_ptr),
            );
        };
    }
}
