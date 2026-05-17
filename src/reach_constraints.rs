use crate::ffi;
use crate::private::handle_type;

handle_type!(ReachConstraints);

impl ReachConstraints {
    #[must_use]
    pub fn new(lower_angle_limit: f64, upper_angle_limit: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_reach_constraints_new(lower_angle_limit, upper_angle_limit)) }
    }

    #[must_use]
    pub fn lower_angle_limit(&self) -> f64 {
        unsafe { ffi::sk_reach_constraints_get_lower_angle_limit(self.ptr) }
    }

    pub fn set_lower_angle_limit(&self, value: f64) {
        unsafe { ffi::sk_reach_constraints_set_lower_angle_limit(self.ptr, value) };
    }

    #[must_use]
    pub fn upper_angle_limit(&self) -> f64 {
        unsafe { ffi::sk_reach_constraints_get_upper_angle_limit(self.ptr) }
    }

    pub fn set_upper_angle_limit(&self, value: f64) {
        unsafe { ffi::sk_reach_constraints_set_upper_angle_limit(self.ptr, value) };
    }
}
