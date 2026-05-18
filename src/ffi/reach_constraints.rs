use core::ffi::c_void;

extern "C" {
    /// Wraps `SKReachConstraints`.
    pub fn sk_reach_constraints_new(lower_angle_limit: f64, upper_angle_limit: f64) -> *mut c_void;
    /// Wraps `SKReachConstraints`.
    pub fn sk_reach_constraints_get_lower_angle_limit(constraints: *mut c_void) -> f64;
    /// Wraps `SKReachConstraints`.
    pub fn sk_reach_constraints_set_lower_angle_limit(constraints: *mut c_void, value: f64);
    /// Wraps `SKReachConstraints`.
    pub fn sk_reach_constraints_get_upper_angle_limit(constraints: *mut c_void) -> f64;
    /// Wraps `SKReachConstraints`.
    pub fn sk_reach_constraints_set_upper_angle_limit(constraints: *mut c_void, value: f64);
}
