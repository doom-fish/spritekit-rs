use apple_cf::cg::{CGPoint, CGRect};

use crate::action::Action;
use crate::constraint::Constraint;
use crate::ffi;
use crate::physics_body::PhysicsBody;
use crate::private::{cstring_from_str, handle_type};
use crate::reach_constraints::ReachConstraints;

handle_type!(Node);

pub trait AsNode {
    #[doc(hidden)]
    fn as_node_ptr(&self) -> *mut core::ffi::c_void;
}

impl AsNode for Node {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

pub trait NodeExt: AsNode {
    fn add_child<C: AsNode>(&self, child: &C) {
        unsafe { ffi::sk_node_add_child(self.as_node_ptr(), child.as_node_ptr()) };
    }

    fn move_to_parent<P: AsNode>(&self, parent: &P) {
        unsafe { ffi::sk_node_move_to_parent(self.as_node_ptr(), parent.as_node_ptr()) };
    }

    fn remove_from_parent(&self) {
        unsafe { ffi::sk_node_remove_from_parent(self.as_node_ptr()) };
    }

    fn remove_all_children(&self) {
        unsafe { ffi::sk_node_remove_all_children(self.as_node_ptr()) };
    }

    #[must_use]
    fn name(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_node_copy_name(self.as_node_ptr())) }
    }

    fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::sk_node_set_name(self.as_node_ptr(), name.as_ptr()) };
        }
    }

    #[must_use]
    fn frame(&self) -> CGRect {
        CGRect::new(
            unsafe { ffi::sk_node_get_frame_x(self.as_node_ptr()) },
            unsafe { ffi::sk_node_get_frame_y(self.as_node_ptr()) },
            unsafe { ffi::sk_node_get_frame_w(self.as_node_ptr()) },
            unsafe { ffi::sk_node_get_frame_h(self.as_node_ptr()) },
        )
    }

    #[must_use]
    fn accumulated_frame(&self) -> CGRect {
        CGRect::new(
            unsafe { ffi::sk_node_calculate_accumulated_frame_x(self.as_node_ptr()) },
            unsafe { ffi::sk_node_calculate_accumulated_frame_y(self.as_node_ptr()) },
            unsafe { ffi::sk_node_calculate_accumulated_frame_w(self.as_node_ptr()) },
            unsafe { ffi::sk_node_calculate_accumulated_frame_h(self.as_node_ptr()) },
        )
    }

    #[must_use]
    fn position(&self) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_node_get_position_x(self.as_node_ptr()) },
            unsafe { ffi::sk_node_get_position_y(self.as_node_ptr()) },
        )
    }

    fn set_position(&self, position: CGPoint) {
        unsafe { ffi::sk_node_set_position(self.as_node_ptr(), position.x, position.y) };
    }

    #[must_use]
    fn z_position(&self) -> f64 {
        unsafe { ffi::sk_node_get_z_position(self.as_node_ptr()) }
    }

    fn set_z_position(&self, z: f64) {
        unsafe { ffi::sk_node_set_z_position(self.as_node_ptr(), z) };
    }

    #[must_use]
    fn z_rotation(&self) -> f64 {
        unsafe { ffi::sk_node_get_z_rotation(self.as_node_ptr()) }
    }

    fn set_z_rotation(&self, rotation: f64) {
        unsafe { ffi::sk_node_set_z_rotation(self.as_node_ptr(), rotation) };
    }

    #[must_use]
    fn x_scale(&self) -> f64 {
        unsafe { ffi::sk_node_get_x_scale(self.as_node_ptr()) }
    }

    fn set_x_scale(&self, scale: f64) {
        unsafe { ffi::sk_node_set_x_scale(self.as_node_ptr(), scale) };
    }

    #[must_use]
    fn y_scale(&self) -> f64 {
        unsafe { ffi::sk_node_get_y_scale(self.as_node_ptr()) }
    }

    fn set_y_scale(&self, scale: f64) {
        unsafe { ffi::sk_node_set_y_scale(self.as_node_ptr(), scale) };
    }

    fn set_scale(&self, scale: f64) {
        unsafe { ffi::sk_node_set_scale(self.as_node_ptr(), scale) };
    }

    #[must_use]
    fn alpha(&self) -> f64 {
        unsafe { ffi::sk_node_get_alpha(self.as_node_ptr()) }
    }

    fn set_alpha(&self, alpha: f64) {
        unsafe { ffi::sk_node_set_alpha(self.as_node_ptr(), alpha) };
    }

    #[must_use]
    fn is_hidden(&self) -> bool {
        unsafe { ffi::sk_node_get_hidden(self.as_node_ptr()) }
    }

    fn set_hidden(&self, hidden: bool) {
        unsafe { ffi::sk_node_set_hidden(self.as_node_ptr(), hidden) };
    }

    #[must_use]
    fn is_paused(&self) -> bool {
        unsafe { ffi::sk_node_get_paused(self.as_node_ptr()) }
    }

    fn set_paused(&self, paused: bool) {
        unsafe { ffi::sk_node_set_paused(self.as_node_ptr(), paused) };
    }

    #[must_use]
    fn speed(&self) -> f64 {
        unsafe { ffi::sk_node_get_speed(self.as_node_ptr()) }
    }

    fn set_speed(&self, speed: f64) {
        unsafe { ffi::sk_node_set_speed(self.as_node_ptr(), speed) };
    }

    #[must_use]
    fn is_user_interaction_enabled(&self) -> bool {
        unsafe { ffi::sk_node_get_user_interaction_enabled(self.as_node_ptr()) }
    }

    fn set_user_interaction_enabled(&self, enabled: bool) {
        unsafe { ffi::sk_node_set_user_interaction_enabled(self.as_node_ptr(), enabled) };
    }

    #[must_use]
    fn is_accessibility_element(&self) -> bool {
        unsafe { ffi::sk_node_get_accessibility_element(self.as_node_ptr()) }
    }

    fn set_accessibility_element(&self, accessibility_element: bool) {
        unsafe {
            ffi::sk_node_set_accessibility_element(self.as_node_ptr(), accessibility_element)
        };
    }

    #[must_use]
    fn accessibility_label(&self) -> Option<String> {
        unsafe {
            crate::error::take_string(ffi::sk_node_copy_accessibility_label(self.as_node_ptr()))
        }
    }

    fn set_accessibility_label(&self, label: &str) {
        if let Some(label) = cstring_from_str(label) {
            unsafe { ffi::sk_node_set_accessibility_label(self.as_node_ptr(), label.as_ptr()) };
        }
    }

    #[must_use]
    fn is_accessibility_enabled(&self) -> bool {
        unsafe { ffi::sk_node_get_accessibility_enabled(self.as_node_ptr()) }
    }

    fn set_accessibility_enabled(&self, accessibility_enabled: bool) {
        unsafe {
            ffi::sk_node_set_accessibility_enabled(self.as_node_ptr(), accessibility_enabled)
        };
    }

    #[must_use]
    fn children_count(&self) -> usize {
        unsafe { ffi::sk_node_get_children_count(self.as_node_ptr()) }
    }

    #[must_use]
    fn physics_body(&self) -> Option<PhysicsBody> {
        unsafe { PhysicsBody::from_raw(ffi::sk_node_get_physics_body(self.as_node_ptr())) }
    }

    fn set_physics_body(&self, body: Option<&PhysicsBody>) {
        unsafe {
            ffi::sk_node_set_physics_body(
                self.as_node_ptr(),
                body.map_or(core::ptr::null_mut(), PhysicsBody::as_ptr),
            );
        };
    }

    #[must_use]
    fn reach_constraints(&self) -> Option<ReachConstraints> {
        unsafe {
            ReachConstraints::from_raw(ffi::sk_node_get_reach_constraints(self.as_node_ptr()))
        }
    }

    fn set_reach_constraints(&self, constraints: Option<&ReachConstraints>) {
        unsafe {
            ffi::sk_node_set_reach_constraints(
                self.as_node_ptr(),
                constraints.map_or(core::ptr::null_mut(), ReachConstraints::as_ptr),
            );
        };
    }

    #[must_use]
    fn constraint_count(&self) -> usize {
        unsafe { ffi::sk_node_get_constraints_count(self.as_node_ptr()) }
    }

    fn set_constraints(&self, constraints: &[&Constraint]) {
        let mut raw: Vec<*mut core::ffi::c_void> = constraints
            .iter()
            .map(|constraint| constraint.as_ptr())
            .collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe { ffi::sk_node_set_constraints(self.as_node_ptr(), raw_ptr, raw.len()) };
    }

    fn clear_constraints(&self) {
        self.set_constraints(&[]);
    }

    fn run_action(&self, action: &Action) {
        unsafe { ffi::sk_node_run_action(self.as_node_ptr(), action.as_ptr()) };
    }

    #[must_use]
    fn has_actions(&self) -> bool {
        unsafe { ffi::sk_node_has_actions(self.as_node_ptr()) }
    }

    fn remove_all_actions(&self) {
        unsafe { ffi::sk_node_remove_all_actions(self.as_node_ptr()) };
    }

    #[must_use]
    fn contains_point(&self, point: CGPoint) -> bool {
        unsafe { ffi::sk_node_contains_point(self.as_node_ptr(), point.x, point.y) }
    }
}

impl<T: AsNode> NodeExt for T {}

impl Node {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_node_new()) }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new().expect("SKNode creation failed")
    }
}
