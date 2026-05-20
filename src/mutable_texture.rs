use core::{ffi::c_void, slice};
use std::panic::{catch_unwind, AssertUnwindSafe};

use apple_cf::cg::CGSize;

use crate::ffi;
use crate::private::handle_type;

handle_type!(MutableTexture);

type MutableTextureModifyCallback = Box<dyn FnOnce(&mut [u8]) + Send + 'static>;

struct MutableTextureModifyContext {
    callback: Option<MutableTextureModifyCallback>,
}

extern "C" fn mutable_texture_modify_pixel_data(
    context: *mut c_void,
    pixel_data: *mut c_void,
    length: usize,
) {
    if context.is_null() {
        return;
    }

    let mut context = unsafe { Box::from_raw(context.cast::<MutableTextureModifyContext>()) };
    let Some(callback) = context.callback.take() else {
        return;
    };
    let pixels = if pixel_data.is_null() || length == 0 {
        &mut []
    } else {
        unsafe { slice::from_raw_parts_mut(pixel_data.cast::<u8>(), length) }
    };
    let _ = catch_unwind(AssertUnwindSafe(|| callback(pixels)));
}

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

    /// Wraps `SKMutableTexture`.
    ///
    /// `SpriteKit` invokes the callback on an arbitrary queue when the texture bytes can be safely
    /// mutated.
    pub fn modify_pixel_data<F>(&self, modify: F)
    where
        F: FnOnce(&mut [u8]) + Send + 'static,
    {
        let context = Box::new(MutableTextureModifyContext {
            callback: Some(Box::new(modify)),
        });
        unsafe {
            ffi::sk_mutable_texture_modify_pixel_data(
                self.ptr,
                Box::into_raw(context).cast(),
                Some(mutable_texture_modify_pixel_data),
            );
        };
    }

    /// Returns a property exposed by `SKMutableTexture`.
    #[must_use]
    pub fn size(&self) -> CGSize {
        CGSize::new(unsafe { ffi::sk_texture_get_size_w(self.ptr) }, unsafe {
            ffi::sk_texture_get_size_h(self.ptr)
        })
    }
}
