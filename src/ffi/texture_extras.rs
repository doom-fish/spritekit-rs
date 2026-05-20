use core::ffi::{c_char, c_void};

extern "C" {
    /// Wraps `SKMutableTexture`.
    pub fn sk_mutable_texture_new_with_size(width: f64, height: f64) -> *mut c_void;
    /// Wraps `SKMutableTexture`.
    pub fn sk_mutable_texture_new_with_size_pixel_format(
        width: f64,
        height: f64,
        pixel_format: i32,
    ) -> *mut c_void;
    /// Wraps `SKMutableTexture`.
    pub fn sk_mutable_texture_modify_pixel_data(
        texture: *mut c_void,
        context: *mut c_void,
        modify: Option<extern "C" fn(*mut c_void, *mut c_void, usize)>,
    );

    /// Wraps `SKTextureAtlas`.
    pub fn sk_texture_atlas_named(name: *const c_char) -> *mut c_void;
    /// Returns a property exposed by `SKTextureAtlas`.
    pub fn sk_texture_atlas_get_texture_names_count(atlas: *mut c_void) -> usize;
    /// Wraps `SKTextureAtlas`.
    pub fn sk_texture_atlas_texture_named(
        atlas: *mut c_void,
        name: *const c_char,
    ) -> *mut c_void;
    /// Wraps `SKTextureAtlas`.
    pub fn sk_texture_atlas_preload_texture_atlases(
        atlases: *mut c_void,
        count: usize,
        context: *mut c_void,
        completion: Option<extern "C" fn(*mut c_void)>,
    );
    /// Wraps `SKTextureAtlas`.
    pub fn sk_texture_atlas_preload_texture_atlases_named(
        atlas_names: *mut c_void,
        count: usize,
        context: *mut c_void,
        completion: Option<extern "C" fn(*mut c_void, *mut c_char, *mut *mut c_void, usize)>,
    );
    /// Wraps `SKTextureAtlas`.
    pub fn sk_texture_atlas_preload(
        atlas: *mut c_void,
        context: *mut c_void,
        completion: Option<extern "C" fn(*mut c_void)>,
    );
}
