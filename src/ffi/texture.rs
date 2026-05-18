use core::ffi::{c_char, c_void};

extern "C" {
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_image_named(name: *const c_char) -> *mut c_void;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_from_cg_image(image: *mut c_void) -> *mut c_void;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_from_rgba_bytes(
        bytes: *const u8,
        length: usize,
        width: usize,
        height: usize,
    ) -> *mut c_void;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_subrect(
        texture: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_get_size_w(texture: *mut c_void) -> f64;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_get_size_h(texture: *mut c_void) -> f64;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_get_filtering_mode(texture: *mut c_void) -> i32;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_set_filtering_mode(texture: *mut c_void, mode: i32);
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_get_uses_mipmaps(texture: *mut c_void) -> bool;
    /// Wraps `SKTexture` and `SKTextureFilteringMode`.
    pub fn sk_texture_set_uses_mipmaps(texture: *mut c_void, uses: bool);
}
