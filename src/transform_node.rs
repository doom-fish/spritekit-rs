use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(TransformNode);

impl AsNode for TransformNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl TransformNode {
    /// Wraps `SKTransformNode`.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_transform_node_new()) }
    }

    /// Wraps `SKTransformNode`.
    #[must_use]
    pub fn x_rotation(&self) -> f64 {
        unsafe { ffi::sk_transform_node_get_x_rotation(self.ptr) }
    }

    /// Sets a property exposed by `SKTransformNode`.
    pub fn set_x_rotation(&self, value: f64) {
        unsafe { ffi::sk_transform_node_set_x_rotation(self.ptr, value) };
    }

    /// Wraps `SKTransformNode`.
    #[must_use]
    pub fn y_rotation(&self) -> f64 {
        unsafe { ffi::sk_transform_node_get_y_rotation(self.ptr) }
    }

    /// Sets a property exposed by `SKTransformNode`.
    pub fn set_y_rotation(&self, value: f64) {
        unsafe { ffi::sk_transform_node_set_y_rotation(self.ptr, value) };
    }
}
