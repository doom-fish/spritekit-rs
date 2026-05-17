use crate::ffi;
use crate::node::AsNode;
use crate::private::{cstring_from_str, handle_type};

handle_type!(ReferenceNode);

impl AsNode for ReferenceNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl ReferenceNode {
    #[must_use]
    pub fn with_file_named(file_name: &str) -> Option<Self> {
        let file_name = cstring_from_str(file_name)?;
        unsafe { Self::from_raw(ffi::sk_reference_node_new_with_file_named(file_name.as_ptr())) }
    }

    #[must_use]
    pub fn with_url_path(path: &str) -> Option<Self> {
        let path = cstring_from_str(path)?;
        unsafe { Self::from_raw(ffi::sk_reference_node_new_with_url_path(path.as_ptr())) }
    }

    pub fn resolve_reference_node(&self) {
        unsafe { ffi::sk_reference_node_resolve(self.ptr) };
    }
}
