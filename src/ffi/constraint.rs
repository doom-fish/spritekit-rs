use core::ffi::c_void;

extern "C" {
    pub fn sk_range_new(lower: f64, upper: f64) -> *mut c_void;
    pub fn sk_range_with_lower_limit(lower: f64) -> *mut c_void;
    pub fn sk_range_with_upper_limit(upper: f64) -> *mut c_void;
    pub fn sk_range_with_constant(value: f64) -> *mut c_void;
    pub fn sk_range_with_variance(value: f64, variance: f64) -> *mut c_void;
    pub fn sk_range_with_no_limits() -> *mut c_void;
    pub fn sk_range_get_lower_limit(range: *mut c_void) -> f64;
    pub fn sk_range_set_lower_limit(range: *mut c_void, lower: f64);
    pub fn sk_range_get_upper_limit(range: *mut c_void) -> f64;
    pub fn sk_range_set_upper_limit(range: *mut c_void, upper: f64);

    pub fn sk_constraint_position_x(range: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_position_y(range: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_position_xy(x_range: *mut c_void, y_range: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_distance_to_node(range: *mut c_void, node: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_distance_to_point(range: *mut c_void, x: f64, y: f64) -> *mut c_void;
    pub fn sk_constraint_distance_to_point_in_node(
        range: *mut c_void,
        x: f64,
        y: f64,
        node: *mut c_void,
    ) -> *mut c_void;
    pub fn sk_constraint_z_rotation(range: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_orient_to_node(node: *mut c_void, offset: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_orient_to_point(x: f64, y: f64, offset: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_orient_to_point_in_node(
        x: f64,
        y: f64,
        node: *mut c_void,
        offset: *mut c_void,
    ) -> *mut c_void;
    pub fn sk_constraint_get_enabled(constraint: *mut c_void) -> bool;
    pub fn sk_constraint_set_enabled(constraint: *mut c_void, enabled: bool);
    pub fn sk_constraint_get_reference_node(constraint: *mut c_void) -> *mut c_void;
    pub fn sk_constraint_set_reference_node(constraint: *mut c_void, node: *mut c_void);
}
