use apple_cf::cg::CGPoint;

use crate::ffi;
use crate::node::{AsNode, Node};
use crate::private::handle_type;

handle_type!(ConstraintRange);
handle_type!(Constraint);

impl ConstraintRange {
    #[must_use]
    pub fn with_limits(lower: f64, upper: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_range_new(lower, upper)) }
    }

    #[must_use]
    pub fn with_lower_limit(lower: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_range_with_lower_limit(lower)) }
    }

    #[must_use]
    pub fn with_upper_limit(upper: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_range_with_upper_limit(upper)) }
    }

    #[must_use]
    pub fn with_constant(value: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_range_with_constant(value)) }
    }

    #[must_use]
    pub fn with_variance(value: f64, variance: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_range_with_variance(value, variance)) }
    }

    #[must_use]
    pub fn no_limits() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_range_with_no_limits()) }
    }

    #[must_use]
    pub fn lower_limit(&self) -> f64 {
        unsafe { ffi::sk_range_get_lower_limit(self.ptr) }
    }

    pub fn set_lower_limit(&self, lower: f64) {
        unsafe { ffi::sk_range_set_lower_limit(self.ptr, lower) };
    }

    #[must_use]
    pub fn upper_limit(&self) -> f64 {
        unsafe { ffi::sk_range_get_upper_limit(self.ptr) }
    }

    pub fn set_upper_limit(&self, upper: f64) {
        unsafe { ffi::sk_range_set_upper_limit(self.ptr, upper) };
    }
}

impl Constraint {
    #[must_use]
    pub fn position_x(range: &ConstraintRange) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_position_x(range.as_ptr())) }
    }

    #[must_use]
    pub fn position_y(range: &ConstraintRange) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_position_y(range.as_ptr())) }
    }

    #[must_use]
    pub fn position_xy(x_range: &ConstraintRange, y_range: &ConstraintRange) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_position_xy(x_range.as_ptr(), y_range.as_ptr())) }
    }

    #[must_use]
    pub fn distance_to_node<N: AsNode>(range: &ConstraintRange, node: &N) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_distance_to_node(range.as_ptr(), node.as_node_ptr())) }
    }

    #[must_use]
    pub fn distance_to_point(range: &ConstraintRange, point: CGPoint) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_distance_to_point(range.as_ptr(), point.x, point.y)) }
    }

    #[must_use]
    pub fn distance_to_point_in_node<N: AsNode>(
        range: &ConstraintRange,
        point: CGPoint,
        node: &N,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_constraint_distance_to_point_in_node(
                range.as_ptr(),
                point.x,
                point.y,
                node.as_node_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn z_rotation(range: &ConstraintRange) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_z_rotation(range.as_ptr())) }
    }

    #[must_use]
    pub fn orient_to_node<N: AsNode>(node: &N, offset: &ConstraintRange) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_orient_to_node(node.as_node_ptr(), offset.as_ptr())) }
    }

    #[must_use]
    pub fn orient_to_point(point: CGPoint, offset: &ConstraintRange) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_constraint_orient_to_point(point.x, point.y, offset.as_ptr())) }
    }

    #[must_use]
    pub fn orient_to_point_in_node<N: AsNode>(
        point: CGPoint,
        node: &N,
        offset: &ConstraintRange,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_constraint_orient_to_point_in_node(
                point.x,
                point.y,
                node.as_node_ptr(),
                offset.as_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::sk_constraint_get_enabled(self.ptr) }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe { ffi::sk_constraint_set_enabled(self.ptr, enabled) };
    }

    #[must_use]
    pub fn reference_node(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::sk_constraint_get_reference_node(self.ptr)) }
    }

    pub fn set_reference_node<N: AsNode>(&self, node: Option<&N>) {
        unsafe {
            ffi::sk_constraint_set_reference_node(
                self.ptr,
                node.map_or(core::ptr::null_mut(), AsNode::as_node_ptr),
            );
        };
    }
}
