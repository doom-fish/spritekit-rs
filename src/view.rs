use core::ffi::c_void;

use apple_cf::cg::CGRect;
use doom_fish_utils::callback_context::CallbackContext;

use crate::error::SpriteKitError;
use crate::ffi;
use crate::node::AsNode;
use crate::private::{handle_type, invoke_callback, is_main_thread, DelegateObject};
use crate::scene::Scene;
use crate::texture::Texture;
use crate::transition::Transition;

handle_type!(View);

type ViewShouldRenderCallback = Box<dyn Fn(f64) -> bool + Send + Sync + 'static>;

struct ViewDelegateContext {
    should_render: Option<ViewShouldRenderCallback>,
}

#[derive(Debug)]
pub struct ViewDelegate {
    inner: DelegateObject<ViewDelegateContext>,
}

unsafe extern "C" fn view_delegate_should_render(context: *mut c_void, time: f64) -> bool {
    unsafe {
        invoke_callback::<ViewDelegateContext, _>(
            context,
            "spritekit::ViewDelegate::should_render",
            |delegate| {
                delegate
                    .should_render
                    .as_ref()
                    .map(|callback| callback(time))
            },
        )
    }
    .flatten()
    .unwrap_or(true)
}

impl ViewDelegate {
    /// Wraps `SKViewDelegate`.
    #[must_use]
    pub fn new() -> Option<Self> {
        Self::create(None)
    }

    /// Wraps `SKViewDelegate`.
    #[must_use]
    pub fn with_should_render<F>(should_render: F) -> Option<Self>
    where
        F: Fn(f64) -> bool + Send + Sync + 'static,
    {
        Self::create(Some(Box::new(should_render)))
    }

    fn create(should_render: Option<ViewShouldRenderCallback>) -> Option<Self> {
        DelegateObject::new(ViewDelegateContext { should_render }, |context| unsafe {
            ffi::sk_view_delegate_new(
                context,
                Some(view_delegate_should_render),
                Some(CallbackContext::<ViewDelegateContext>::RELEASE),
            )
        })
        .map(|inner| Self { inner })
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

impl View {
    /// Wraps `SKView`.
    pub fn with_frame(frame: CGRect) -> Result<Self, SpriteKitError> {
        if !is_main_thread() {
            return Err(SpriteKitError::new(
                "SKView must be created and used on the main thread",
            ));
        }
        unsafe {
            Self::from_raw(ffi::sk_view_new_with_frame(
                frame.origin.x,
                frame.origin.y,
                frame.size.width,
                frame.size.height,
            ))
        }
        .ok_or_else(|| SpriteKitError::new("SKView(frame:) returned nil"))
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn is_paused(&self) -> bool {
        unsafe { ffi::sk_view_get_paused(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_paused(&self, paused: bool) {
        unsafe { ffi::sk_view_set_paused(self.ptr, paused) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn shows_fps(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_fps(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_shows_fps(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_fps(self.ptr, shows) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn shows_draw_count(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_draw_count(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_shows_draw_count(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_draw_count(self.ptr, shows) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn shows_node_count(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_node_count(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_shows_node_count(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_node_count(self.ptr, shows) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn shows_quad_count(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_quad_count(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_shows_quad_count(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_quad_count(self.ptr, shows) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn shows_physics(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_physics(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_shows_physics(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_physics(self.ptr, shows) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn shows_fields(&self) -> bool {
        unsafe { ffi::sk_view_get_shows_fields(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_shows_fields(&self, shows: bool) {
        unsafe { ffi::sk_view_set_shows_fields(self.ptr, shows) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn is_asynchronous(&self) -> bool {
        unsafe { ffi::sk_view_get_asynchronous(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_asynchronous(&self, asynchronous: bool) {
        unsafe { ffi::sk_view_set_asynchronous(self.ptr, asynchronous) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn allows_transparency(&self) -> bool {
        unsafe { ffi::sk_view_get_allows_transparency(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_allows_transparency(&self, allows: bool) {
        unsafe { ffi::sk_view_set_allows_transparency(self.ptr, allows) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn ignores_sibling_order(&self) -> bool {
        unsafe { ffi::sk_view_get_ignores_sibling_order(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_ignores_sibling_order(&self, ignores: bool) {
        unsafe { ffi::sk_view_set_ignores_sibling_order(self.ptr, ignores) };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn should_cull_non_visible_nodes(&self) -> bool {
        unsafe { ffi::sk_view_get_should_cull_non_visible_nodes(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_should_cull_non_visible_nodes(&self, should_cull: bool) {
        unsafe { ffi::sk_view_set_should_cull_non_visible_nodes(self.ptr, should_cull) };
    }

    /// Wraps `SKView`.
    #[must_use]
    pub fn preferred_frames_per_second(&self) -> isize {
        unsafe { ffi::sk_view_get_preferred_frames_per_second(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_preferred_frames_per_second(&self, fps: isize) {
        unsafe { ffi::sk_view_set_preferred_frames_per_second(self.ptr, fps) };
    }

    /// Wraps `SKView`.
    #[must_use]
    pub fn disable_depth_stencil_buffer(&self) -> bool {
        unsafe { ffi::sk_view_get_disable_depth_stencil_buffer(self.ptr) }
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_disable_depth_stencil_buffer(&self, disable: bool) {
        unsafe { ffi::sk_view_set_disable_depth_stencil_buffer(self.ptr, disable) };
    }

    /// Sets a property exposed by `SKView`.
    pub fn set_delegate(&self, delegate: Option<&ViewDelegate>) {
        unsafe {
            ffi::sk_view_set_delegate(
                self.ptr,
                delegate.map_or(core::ptr::null_mut(), ViewDelegate::as_ptr),
            );
        };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::sk_view_has_delegate(self.ptr) }
    }

    /// Wraps `SKView`.
    pub fn present_scene(&self, scene: Option<&Scene>) {
        unsafe {
            ffi::sk_view_present_scene(
                self.ptr,
                scene.map_or(core::ptr::null_mut(), Scene::as_ptr),
            );
        };
    }

    /// Wraps `SKView`.
    pub fn present_scene_with_transition(&self, scene: &Scene, transition: &Transition) {
        unsafe {
            ffi::sk_view_present_scene_with_transition(
                self.ptr,
                scene.as_ptr(),
                transition.as_ptr(),
            );
        };
    }

    /// Returns a property exposed by `SKView`.
    #[must_use]
    pub fn scene(&self) -> Option<Scene> {
        unsafe { Scene::from_raw(ffi::sk_view_get_scene(self.ptr)) }
    }

    /// Wraps `SKView`.
    #[must_use]
    pub fn texture_from_node<N: AsNode>(&self, node: &N) -> Option<Texture> {
        unsafe { Texture::from_raw(ffi::sk_view_texture_from_node(self.ptr, node.as_node_ptr())) }
    }
}
