use crate::ffi;
use crate::private::{cstring_from_str, handle_type};
use crate::texture::Texture;

handle_type!(TextureAtlas);

impl TextureAtlas {
    /// Wraps `SKTextureAtlas`.
    #[must_use]
    pub fn named(name: &str) -> Option<Self> {
        let name = cstring_from_str(name)?;
        unsafe { Self::from_raw(ffi::sk_texture_atlas_named(name.as_ptr())) }
    }

    /// Returns a property exposed by `SKTextureAtlas`.
    #[must_use]
    pub fn texture_names_count(&self) -> usize {
        unsafe { ffi::sk_texture_atlas_get_texture_names_count(self.ptr) }
    }

    /// Wraps `SKTextureAtlas`.
    #[must_use]
    pub fn texture_named(&self, name: &str) -> Option<Texture> {
        let name = cstring_from_str(name)?;
        unsafe { Texture::from_raw(ffi::sk_texture_atlas_texture_named(self.ptr, name.as_ptr())) }
    }
}
