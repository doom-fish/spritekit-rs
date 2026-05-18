use apple_cf::cg::{CGPoint, CGRect, CGSize};

use crate::color::Color;
use crate::ffi;
use crate::node::AsNode;
use crate::private::{cstring_from_str, handle_type};
use crate::shader::Shader;
use crate::texture::Texture;

handle_type!(SpriteNode);

impl AsNode for SpriteNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl SpriteNode {
    /// Wraps `SKSpriteNode`.
    #[must_use]
    pub fn with_texture(texture: Option<&Texture>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_sprite_node_new_with_texture(
                texture.map_or(core::ptr::null_mut(), Texture::as_ptr),
            ))
        }
    }

    /// Wraps `SKSpriteNode`.
    #[must_use]
    pub fn with_color(color: Color, size: CGSize) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_sprite_node_new_with_color(
                color.r,
                color.g,
                color.b,
                color.a,
                size.width,
                size.height,
            ))
        }
    }

    /// Wraps `SKSpriteNode`.
    #[must_use]
    pub fn image_named(name: &str) -> Option<Self> {
        let name = cstring_from_str(name)?;
        unsafe { Self::from_raw(ffi::sk_sprite_node_new_image_named(name.as_ptr())) }
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn texture(&self) -> Option<Texture> {
        unsafe { Texture::from_raw(ffi::sk_sprite_node_get_texture(self.ptr)) }
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_texture(&self, texture: Option<&Texture>) {
        unsafe {
            ffi::sk_sprite_node_set_texture(
                self.ptr,
                texture.map_or(core::ptr::null_mut(), Texture::as_ptr),
            );
        };
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn normal_texture(&self) -> Option<Texture> {
        unsafe { Texture::from_raw(ffi::sk_sprite_node_get_normal_texture(self.ptr)) }
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_normal_texture(&self, texture: Option<&Texture>) {
        unsafe {
            ffi::sk_sprite_node_set_normal_texture(
                self.ptr,
                texture.map_or(core::ptr::null_mut(), Texture::as_ptr),
            );
        };
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_sprite_node_get_size_w(self.ptr) },
            unsafe { ffi::sk_sprite_node_get_size_h(self.ptr) },
        )
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_size(&self, size: CGSize) {
        unsafe { ffi::sk_sprite_node_set_size(self.ptr, size.width, size.height) };
    }

    /// Wraps `SKSpriteNode`.
    pub fn scale_to_size(&self, size: CGSize) {
        unsafe { ffi::sk_sprite_node_scale_to_size(self.ptr, size.width, size.height) };
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn anchor_point(&self) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_sprite_node_get_anchor_x(self.ptr) },
            unsafe { ffi::sk_sprite_node_get_anchor_y(self.ptr) },
        )
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_anchor_point(&self, anchor: CGPoint) {
        unsafe { ffi::sk_sprite_node_set_anchor_point(self.ptr, anchor.x, anchor.y) };
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_color(&self, color: Color) {
        unsafe { ffi::sk_sprite_node_set_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn color_blend_factor(&self) -> f64 {
        unsafe { ffi::sk_sprite_node_get_color_blend_factor(self.ptr) }
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_color_blend_factor(&self, factor: f64) {
        unsafe { ffi::sk_sprite_node_set_color_blend_factor(self.ptr, factor) };
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn blend_mode(&self) -> crate::physics::BlendMode {
        crate::physics::BlendMode::from_raw(unsafe { ffi::sk_sprite_node_get_blend_mode(self.ptr) })
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_blend_mode(&self, mode: crate::physics::BlendMode) {
        unsafe { ffi::sk_sprite_node_set_blend_mode(self.ptr, mode as i32) };
    }

    /// Wraps `SKSpriteNode`.
    #[must_use]
    pub fn lighting_bitmask(&self) -> u32 {
        unsafe { ffi::sk_sprite_node_get_lighting_bitmask(self.ptr) }
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_lighting_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_sprite_node_set_lighting_bitmask(self.ptr, mask) };
    }

    /// Wraps `SKSpriteNode`.
    #[must_use]
    pub fn shadow_cast_bitmask(&self) -> u32 {
        unsafe { ffi::sk_sprite_node_get_shadow_cast_bitmask(self.ptr) }
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_shadow_cast_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_sprite_node_set_shadow_cast_bitmask(self.ptr, mask) };
    }

    /// Wraps `SKSpriteNode`.
    #[must_use]
    pub fn shadowed_bitmask(&self) -> u32 {
        unsafe { ffi::sk_sprite_node_get_shadowed_bitmask(self.ptr) }
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_shadowed_bitmask(&self, mask: u32) {
        unsafe { ffi::sk_sprite_node_set_shadowed_bitmask(self.ptr, mask) };
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn center_rect(&self) -> CGRect {
        CGRect::new(
            unsafe { ffi::sk_sprite_node_get_center_rect_x(self.ptr) },
            unsafe { ffi::sk_sprite_node_get_center_rect_y(self.ptr) },
            unsafe { ffi::sk_sprite_node_get_center_rect_w(self.ptr) },
            unsafe { ffi::sk_sprite_node_get_center_rect_h(self.ptr) },
        )
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_center_rect(&self, rect: CGRect) {
        unsafe {
            ffi::sk_sprite_node_set_center_rect(
                self.ptr,
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            );
        };
    }

    /// Returns a property exposed by `SKSpriteNode`.
    #[must_use]
    pub fn shader(&self) -> Option<Shader> {
        unsafe { Shader::from_raw(ffi::sk_sprite_node_get_shader(self.ptr)) }
    }

    /// Sets a property exposed by `SKSpriteNode`.
    pub fn set_shader(&self, shader: Option<&Shader>) {
        unsafe {
            ffi::sk_sprite_node_set_shader(
                self.ptr,
                shader.map_or(core::ptr::null_mut(), Shader::as_ptr),
            );
        };
    }
}
