use apple_cf::cg::CGRect;

use crate::ffi;
use crate::node::AsNode;
use crate::private::handle_type;
use crate::scene::Scene;
use crate::texture::Texture;

handle_type!(View);

impl View {
    #[must_use]
    pub fn with_frame(frame: CGRect) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::sk_view_new_with_frame(
                frame.x,
                frame.y,
                frame.width,
                frame.height,
            ))
        }
    }

    #[must_use]
    pub fn is_paused(&self) -> bool {
        unsafe { ffi::sk_view_get_paused(self.ptr) }
    }

    pub fn set_paused(&self, paused: bool) {
        unsafe { ffi::sk_view_set_paused(self.ptr, paused) };
    }

    #[must_use]
    pub fn shows_fps(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_fps(self.ptr) }
    }

    pub fn set_shows_fps(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_fps(self.ptr, shows) };
    }

    #[must_use]
    pub fn shows_draw_count(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_draw_count(self.ptr) }
    }

    pub fn set_shows_draw_count(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_draw_count(self.ptr, shows) };
    }

    #[must_use]
    pub fn shows_node_count(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_node_count(self.ptr) }
    }

    pub fn set_shows_node_count(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_node_count(self.ptr, shows) };
    }

    #[must_use]
    pub fn shows_quad_count(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_quad_count(self.ptr) }
    }

    pub fn set_shows_quad_count(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_quad_count(self.ptr, shows) };
    }

    #[must_use]
    pub fn shows_physics(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_physics(self.ptr) }
    }

    pub fn set_shows_physics(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_physics(self.ptr, shows) };
    }

    #[must_use]
    pub fn shows_fields(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_fields(self.ptr) }
    }

    pub fn set_shows_fields(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_fields(self.ptr, shows) };
    }

    #[must_use]
    pub fn is_asynchronous(&self) -> bool {
        unsafe { ffi::sk_view_get_asynchronous(self.ptr) }
    }

    pub fn set_asynchronous(&self, asynchronous: bool) {
        unsafe { ffi::sk_view_set_asynchronous(self.ptr, asynchronous) };
    }

    #[must_use]
    pub fn allows_transparency(&self) -> bool {
        unsafe { ffi::sk_view_get_allows_transparency(self.ptr) }
    }

    pub fn set_allows_transparency(&self, allows: bool) {
        unsafe { ffi::sk_view_set_allows_transparency(self.ptr, allows) };
    }

    #[must_use]
    pub fn ignores_sibling_order(&self) -> bool {
        unsafe { ffi::sk_view_get_ignores_sibling_order(self.ptr) }
    }

    pub fn set_ignores_sibling_order(&self, ignores: bool) {
        unsafe { ffi::sk_view_set_ignores_sibling_order(self.ptr, ignores) };
    }

    #[must_use]
    pub fn should_cull_non_visible_nodes(&self) -> bool {
        unsafe { ffi::sk_view_get_should_cull_non_visible_nodes(self.ptr) }
    }

    pub fn set_should_cull_non_visible_nodes(&self, should_cull: bool) {
        unsafe { ffi::sk_view_set_should_cull_non_visible_nodes(self.ptr, should_cull) };
    }

    #[must_use]
    pub fn preferred_frames_per_second(&self) -> isize {
        unsafe { ffi::sk_view_get_preferred_frames_per_second(self.ptr) }
    }

    pub fn set_preferred_frames_per_second(&self, fps: isize) {
        unsafe { ffi::sk_view_set_preferred_frames_per_second(self.ptr, fps) };
    }

    #[must_use]
    pub fn disable_depth_stencil_buffer(&self) -> bool {
        unsafe { ffi::sk_view_get_disable_depth_stencil_buffer(self.ptr) }
    }

    pub fn set_disable_depth_stencil_buffer(&self, disable: bool) {
        unsafe { ffi::sk_view_set_disable_depth_stencil_buffer(self.ptr, disable) };
    }

    pub fn present_scene(&self, scene: Option<&Scene>) {
        unsafe {
            ffi::sk_view_present_scene(
                self.ptr,
                scene.map_or(core::ptr::null_mut(), Scene::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn scene(&self) -> Option<Scene> {
        unsafe { Scene::from_raw(ffi::sk_view_get_scene(self.ptr)) }
    }

    #[must_use]
    pub fn texture_from_node<N: AsNode>(&self, node: &N) -> Option<Texture> {
        unsafe { Texture::from_raw(ffi::sk_view_texture_from_node(self.ptr, node.as_node_ptr())) }
    }
}
