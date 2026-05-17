use apple_cf::cg::{CGRect, CGSize};

use crate::color::Color;
use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;

handle_type!(ShapeNode);

impl AsNode for ShapeNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl ShapeNode {
    #[must_use]
    pub fn with_rect(rect: CGRect) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_shape_node_new_with_rect(rect.x, rect.y, rect.width, rect.height)) }
    }

    #[must_use]
    pub fn with_rect_of_size(size: CGSize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_shape_node_new_with_rect_of_size(size.width, size.height)) }
    }

    #[must_use]
    pub fn with_circle(radius: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_shape_node_new_with_circle(radius)) }
    }

    #[must_use]
    pub fn line_width(&self) -> f64 {
        unsafe { ffi::sk_shape_node_get_line_width(self.ptr) }
    }

    pub fn set_line_width(&self, value: f64) {
        unsafe { ffi::sk_shape_node_set_line_width(self.ptr, value) };
    }

    #[must_use]
    pub fn glow_width(&self) -> f64 {
        unsafe { ffi::sk_shape_node_get_glow_width(self.ptr) }
    }

    pub fn set_glow_width(&self, value: f64) {
        unsafe { ffi::sk_shape_node_set_glow_width(self.ptr, value) };
    }

    #[must_use]
    pub fn is_antialiased(&self) -> bool {
        unsafe { ffi::sk_shape_node_get_antialiased(self.ptr) }
    }

    pub fn set_antialiased(&self, antialiased: bool) {
        unsafe { ffi::sk_shape_node_set_antialiased(self.ptr, antialiased) };
    }

    #[must_use]
    pub fn line_length(&self) -> f64 {
        unsafe { ffi::sk_shape_node_get_line_length(self.ptr) }
    }

    pub fn set_stroke_color(&self, color: Color) {
        unsafe { ffi::sk_shape_node_set_stroke_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    pub fn set_fill_color(&self, color: Color) {
        unsafe { ffi::sk_shape_node_set_fill_color(self.ptr, color.r, color.g, color.b, color.a) };
    }
}
