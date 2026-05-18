use apple_cf::cg::CGSize;

use crate::ffi;
use crate::private::handle_type;

handle_type!(MutableTexture);

impl MutableTexture {
    /// Wraps `SKMutableTexture`.
    #[must_use]
    pub fn with_size(size: CGSize) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_mutable_texture_new_with_size(
                size.width,
                size.height,
            ))
        }
    }

    /// Wraps `SKMutableTexture`.
    #[must_use]
    pub fn with_size_pixel_format(size: CGSize, pixel_format: i32) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_mutable_texture_new_with_size_pixel_format(
                size.width,
                size.height,
                pixel_format,
            ))
        }
    }

    /// Returns a property exposed by `SKMutableTexture`.
    #[must_use]
    pub fn size(&self) -> CGSize {
        CGSize::new(unsafe { ffi::sk_texture_get_size_w(self.ptr) }, unsafe {
            ffi::sk_texture_get_size_h(self.ptr)
        })
    }
}
