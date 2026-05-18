use apple_cf::cg::{CGPoint, CGSize};

use crate::ffi;
use crate::private::handle_type;

handle_type!(Region);

impl Region {
    /// Wraps `SKRegion`.
    #[must_use]
    pub fn infinite() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_region_infinite()) }
    }

    /// Wraps `SKRegion`.
    #[must_use]
    pub fn with_radius(radius: f32) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_region_new_with_radius(radius)) }
    }

    /// Wraps `SKRegion`.
    #[must_use]
    pub fn with_size(size: CGSize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_region_new_with_size(size.width, size.height)) }
    }

    /// Wraps `SKRegion`.
    #[must_use]
    pub fn inverse(&self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_region_inverse(self.ptr)) }
    }

    /// Wraps `SKRegion`.
    #[must_use]
    pub fn union(&self, other: &Self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_region_union(self.ptr, other.ptr)) }
    }

    /// Wraps `SKRegion`.
    #[must_use]
    pub fn difference(&self, other: &Self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_region_difference(self.ptr, other.ptr)) }
    }

    /// Wraps `SKRegion`.
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_region_intersection(self.ptr, other.ptr)) }
    }

    /// Wraps `SKRegion`.
    #[must_use]
    pub fn contains_point(&self, point: CGPoint) -> bool {
        unsafe { ffi::sk_region_contains_point(self.ptr, point.x, point.y) }
    }
}
