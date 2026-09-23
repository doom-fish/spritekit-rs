use core::{ffi::c_void, mem::ManuallyDrop, slice};
use std::sync::Mutex;

use apple_cf::cg::CGSize;
use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::panic_safe::catch_user_panic;

use crate::ffi;
use crate::private::handle_type;

handle_type!(MutableTexture);

type PixelDataModifier = Box<dyn FnOnce(&mut [u8]) + Send + 'static>;
type MutableTextureModifyCallback = Mutex<Option<PixelDataModifier>>;

unsafe extern "C" fn mutable_texture_modify_pixel_data(
    context: *mut c_void,
    pixel_data: *mut c_void,
    length: usize,
) {
    const SITE: &str = "spritekit::MutableTexture::modify_pixel_data";
    let callback = unsafe {
        CallbackContext::<MutableTextureModifyCallback>::with(context, SITE, |slot| {
            slot.lock().ok().and_then(|mut slot| slot.take())
        })
    }
    .flatten();
    let Some(callback) = callback else {
        return;
    };
    let pixels = if pixel_data.is_null() || length == 0 {
        &mut []
    } else {
        unsafe { slice::from_raw_parts_mut(pixel_data.cast::<u8>(), length) }
    };
    catch_user_panic(SITE, || callback(pixels));
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
        let modify: PixelDataModifier = Box::new(modify);
        let context = ManuallyDrop::new(CallbackContext::<MutableTextureModifyCallback>::new(
            Mutex::new(Some(modify)),
        ));
        unsafe {
            ffi::sk_mutable_texture_modify_pixel_data(
                self.ptr,
                context.as_ptr(),
                Some(mutable_texture_modify_pixel_data),
                Some(CallbackContext::<MutableTextureModifyCallback>::RELEASE),
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
