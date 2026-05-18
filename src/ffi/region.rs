use core::ffi::c_void;

extern "C" {
    /// Wraps `SKRegion`.
    pub fn sk_region_infinite() -> *mut c_void;
    /// Wraps `SKRegion`.
    pub fn sk_region_new_with_radius(radius: f32) -> *mut c_void;
    /// Wraps `SKRegion`.
    pub fn sk_region_new_with_size(width: f64, height: f64) -> *mut c_void;
    /// Wraps `SKRegion`.
    pub fn sk_region_inverse(region: *mut c_void) -> *mut c_void;
    /// Wraps `SKRegion`.
    pub fn sk_region_union(region: *mut c_void, other: *mut c_void) -> *mut c_void;
    /// Wraps `SKRegion`.
    pub fn sk_region_difference(region: *mut c_void, other: *mut c_void) -> *mut c_void;
    /// Wraps `SKRegion`.
    pub fn sk_region_intersection(region: *mut c_void, other: *mut c_void) -> *mut c_void;
    /// Wraps `SKRegion`.
    pub fn sk_region_contains_point(region: *mut c_void, x: f64, y: f64) -> bool;
}
