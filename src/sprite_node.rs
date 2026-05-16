use apple_cf::cg::{CGPoint, CGSize};

use crate::color::Color;
use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;
use crate::texture::Texture;

handle_type!(SpriteNode);

impl AsNode for SpriteNode {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl SpriteNode {
    /// Creates a sprite with the given texture.
    #[must_use]
    pub fn with_texture(texture: Option<&Texture>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_sprite_node_new_with_texture(
                texture.map_or(core::ptr::null_mut(), Texture::as_ptr),
            ))
        }
    }

    /// Creates a solid-color sprite with the given color and size.
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

    #[must_use]
    pub fn texture(&self) -> Option<Texture> {
        unsafe { Texture::from_raw(ffi::sk_sprite_node_get_texture(self.ptr)) }
    }

    pub fn set_texture(&self, texture: Option<&Texture>) {
        unsafe {
            ffi::sk_sprite_node_set_texture(
                self.ptr,
                texture.map_or(core::ptr::null_mut(), Texture::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn size(&self) -> CGSize {
        let w = unsafe { ffi::sk_sprite_node_get_size_w(self.ptr) };
        let h = unsafe { ffi::sk_sprite_node_get_size_h(self.ptr) };
        CGSize::new(w, h)
    }

    pub fn set_size(&self, size: CGSize) {
        unsafe { ffi::sk_sprite_node_set_size(self.ptr, size.width, size.height) };
    }

    #[must_use]
    pub fn anchor_point(&self) -> CGPoint {
        let x = unsafe { ffi::sk_sprite_node_get_anchor_x(self.ptr) };
        let y = unsafe { ffi::sk_sprite_node_get_anchor_y(self.ptr) };
        CGPoint::new(x, y)
    }

    pub fn set_anchor_point(&self, anchor: CGPoint) {
        unsafe { ffi::sk_sprite_node_set_anchor_point(self.ptr, anchor.x, anchor.y) };
    }

    pub fn set_color(&self, color: Color) {
        unsafe {
            ffi::sk_sprite_node_set_color(self.ptr, color.r, color.g, color.b, color.a);
        };
    }

    #[must_use]
    pub fn color_blend_factor(&self) -> f64 {
        unsafe { ffi::sk_sprite_node_get_color_blend_factor(self.ptr) }
    }

    pub fn set_color_blend_factor(&self, factor: f64) {
        unsafe { ffi::sk_sprite_node_set_color_blend_factor(self.ptr, factor) };
    }

    #[must_use]
    pub fn blend_mode(&self) -> crate::physics::BlendMode {
        crate::physics::BlendMode::from_raw(unsafe { ffi::sk_sprite_node_get_blend_mode(self.ptr) })
    }

    pub fn set_blend_mode(&self, mode: crate::physics::BlendMode) {
        unsafe { ffi::sk_sprite_node_set_blend_mode(self.ptr, mode as i32) };
    }
}
