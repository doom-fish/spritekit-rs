use apple_cf::cg::{CGRect, CGSize};

use crate::ffi;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Texture);

/// `SKTexture` filtering mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextureFilteringMode {
    Nearest = 0,
    Linear = 1,
}

impl TextureFilteringMode {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Nearest,
            _ => Self::Linear,
        }
    }
}

impl Texture {
    /// Loads a texture by asset-catalogue / bundle name.
    #[must_use]
    pub fn image_named(name: &str) -> Option<Self> {
        let name = cstring_from_str(name)?;
        unsafe { Self::from_raw(ffi::sk_texture_image_named(name.as_ptr())) }
    }

    /// Creates a texture from a `CGImage` (takes a borrowed raw pointer from `apple-cf`).
    ///
    /// # Safety
    ///
    /// `image_ptr` must be a valid, live `CGImage` pointer.
    #[must_use]
    pub unsafe fn from_cg_image_ptr(image_ptr: *mut core::ffi::c_void) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_texture_from_cg_image(image_ptr)) }
    }

    /// Creates a texture from raw RGBA bytes.
    ///
    /// `bytes` must have length `width * height * 4`.
    #[must_use]
    pub fn from_rgba_bytes(bytes: &[u8], width: usize, height: usize) -> Option<Self> {
        if bytes.len() < width.saturating_mul(height).saturating_mul(4) {
            return None;
        }
        unsafe {
            Self::from_raw(ffi::sk_texture_from_rgba_bytes(
                bytes.as_ptr(),
                bytes.len(),
                width,
                height,
            ))
        }
    }

    /// Returns a sub-rectangle texture from this texture.
    ///
    /// `rect` coordinates are in normalised (0..1) UV space.
    #[must_use]
    pub fn subrect(&self, rect: CGRect) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_texture_subrect(
                self.ptr,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            ))
        }
    }

    #[must_use]
    pub fn size(&self) -> CGSize {
        let w = unsafe { ffi::sk_texture_get_size_w(self.ptr) };
        let h = unsafe { ffi::sk_texture_get_size_h(self.ptr) };
        CGSize::new(w, h)
    }

    #[must_use]
    pub fn filtering_mode(&self) -> TextureFilteringMode {
        TextureFilteringMode::from_raw(unsafe { ffi::sk_texture_get_filtering_mode(self.ptr) })
    }

    pub fn set_filtering_mode(&self, mode: TextureFilteringMode) {
        unsafe { ffi::sk_texture_set_filtering_mode(self.ptr, mode as i32) };
    }

    #[must_use]
    pub fn uses_mipmaps(&self) -> bool {
        unsafe { ffi::sk_texture_get_uses_mipmaps(self.ptr) }
    }

    pub fn set_uses_mipmaps(&self, uses: bool) {
        unsafe { ffi::sk_texture_set_uses_mipmaps(self.ptr, uses) };
    }
}
