use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;
use crate::region::Region;

handle_type!(FieldNode);

impl AsNode for FieldNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl FieldNode {
    #[must_use]
    pub fn drag() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_field_node_drag()) }
    }

    #[must_use]
    pub fn vortex() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_field_node_vortex()) }
    }

    #[must_use]
    pub fn linear_gravity(direction: [f32; 3]) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_field_node_linear_gravity(
                direction[0],
                direction[1],
                direction[2],
            ))
        }
    }

    #[must_use]
    pub fn region(&self) -> Option<Region> {
        unsafe { Region::from_raw(ffi::sk_field_node_get_region(self.ptr)) }
    }

    pub fn set_region(&self, region: Option<&Region>) {
        unsafe {
            ffi::sk_field_node_set_region(
                self.ptr,
                region.map_or(core::ptr::null_mut(), Region::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn strength(&self) -> f32 {
        unsafe { ffi::sk_field_node_get_strength(self.ptr) }
    }

    pub fn set_strength(&self, value: f32) {
        unsafe { ffi::sk_field_node_set_strength(self.ptr, value) };
    }

    #[must_use]
    pub fn direction(&self) -> [f32; 3] {
        [
            unsafe { ffi::sk_field_node_get_direction_x(self.ptr) },
            unsafe { ffi::sk_field_node_get_direction_y(self.ptr) },
            unsafe { ffi::sk_field_node_get_direction_z(self.ptr) },
        ]
    }

    pub fn set_direction(&self, direction: [f32; 3]) {
        unsafe {
            ffi::sk_field_node_set_direction(self.ptr, direction[0], direction[1], direction[2])
        };
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::sk_field_node_get_enabled(self.ptr) }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe { ffi::sk_field_node_set_enabled(self.ptr, enabled) };
    }
}
