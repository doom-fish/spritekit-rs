use core::ffi::c_void;

use apple_cf::cg::{CGPoint, CGSize};
use doom_fish_utils::callback_context::CallbackContext;

use crate::camera_node::CameraNode;
use crate::ffi;
use crate::node::AsNode;
use crate::physics_world::PhysicsWorld;
use crate::private::{handle_type, invoke_callback, DelegateObject};
use crate::view::View;

handle_type!(Scene);

type SceneUpdateCallback = Box<dyn Fn(f64) + Send + Sync + 'static>;
type SceneStepCallback = Box<dyn Fn() + Send + Sync + 'static>;

#[derive(Default)]
pub struct SceneDelegateCallbacks {
    update: Option<SceneUpdateCallback>,
    did_evaluate_actions: Option<SceneStepCallback>,
    did_simulate_physics: Option<SceneStepCallback>,
    did_apply_constraints: Option<SceneStepCallback>,
    did_finish_update: Option<SceneStepCallback>,
}

impl SceneDelegateCallbacks {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            update: None,
            did_evaluate_actions: None,
            did_simulate_physics: None,
            did_apply_constraints: None,
            did_finish_update: None,
        }
    }

    #[must_use]
    pub fn on_update<F>(mut self, callback: F) -> Self
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        self.update = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_evaluate_actions<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.did_evaluate_actions = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_simulate_physics<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.did_simulate_physics = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_apply_constraints<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.did_apply_constraints = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_finish_update<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.did_finish_update = Some(Box::new(callback));
        self
    }
}

#[derive(Debug)]
pub struct SceneDelegate {
    inner: DelegateObject<SceneDelegateCallbacks>,
}

unsafe extern "C" fn scene_delegate_update(context: *mut c_void, current_time: f64) {
    unsafe {
        invoke_callback::<SceneDelegateCallbacks, _>(
            context,
            "spritekit::SceneDelegate::update",
            |callbacks| {
                if let Some(callback) = &callbacks.update {
                    callback(current_time);
                }
            },
        );
    }
}

unsafe fn scene_delegate_step(
    context: *mut c_void,
    site: &str,
    select: fn(&SceneDelegateCallbacks) -> Option<&SceneStepCallback>,
) {
    unsafe {
        invoke_callback::<SceneDelegateCallbacks, _>(context, site, |callbacks| {
            if let Some(callback) = select(callbacks) {
                callback();
            }
        });
    }
}

unsafe extern "C" fn scene_delegate_did_evaluate_actions(context: *mut c_void) {
    unsafe {
        scene_delegate_step(
            context,
            "spritekit::SceneDelegate::did_evaluate_actions",
            |callbacks| callbacks.did_evaluate_actions.as_ref(),
        );
    }
}

unsafe extern "C" fn scene_delegate_did_simulate_physics(context: *mut c_void) {
    unsafe {
        scene_delegate_step(
            context,
            "spritekit::SceneDelegate::did_simulate_physics",
            |callbacks| callbacks.did_simulate_physics.as_ref(),
        );
    }
}

unsafe extern "C" fn scene_delegate_did_apply_constraints(context: *mut c_void) {
    unsafe {
        scene_delegate_step(
            context,
            "spritekit::SceneDelegate::did_apply_constraints",
            |callbacks| callbacks.did_apply_constraints.as_ref(),
        );
    }
}

unsafe extern "C" fn scene_delegate_did_finish_update(context: *mut c_void) {
    unsafe {
        scene_delegate_step(
            context,
            "spritekit::SceneDelegate::did_finish_update",
            |callbacks| callbacks.did_finish_update.as_ref(),
        );
    }
}

/// Enum for `SKSceneScaleMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SceneScaleMode {
    Fill = 0,
    AspectFill = 1,
    AspectFit = 2,
    ResizeFill = 3,
}

impl SceneScaleMode {
    /// Converts a raw value from `SKSceneScaleMode`.
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Fill,
            1 => Self::AspectFill,
            3 => Self::ResizeFill,
            _ => Self::AspectFit,
        }
    }
}

impl AsNode for Scene {
    fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl SceneDelegate {
    /// Wraps `SKSceneDelegate`.
    #[must_use]
    pub fn new() -> Option<Self> {
        Self::from_callbacks(SceneDelegateCallbacks::new())
    }

    /// Wraps `SKSceneDelegate`.
    #[must_use]
    pub fn with_update<F>(update: F) -> Option<Self>
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        Self::from_callbacks(SceneDelegateCallbacks::new().on_update(update))
    }

    #[must_use]
    pub fn from_callbacks(callbacks: SceneDelegateCallbacks) -> Option<Self> {
        DelegateObject::new(callbacks, |context| unsafe {
            ffi::sk_scene_delegate_new(
                context,
                Some(scene_delegate_update),
                Some(scene_delegate_did_evaluate_actions),
                Some(scene_delegate_did_simulate_physics),
                Some(scene_delegate_did_apply_constraints),
                Some(scene_delegate_did_finish_update),
                Some(CallbackContext::<SceneDelegateCallbacks>::RELEASE),
            )
        })
        .map(|inner| Self { inner })
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

impl Scene {
    /// Wraps `SKScene`.
    #[must_use]
    pub fn with_size(size: CGSize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_scene_new_with_size(size.width, size.height)) }
    }

    /// Returns a property exposed by `SKScene`.
    #[must_use]
    pub fn size(&self) -> CGSize {
        CGSize::new(unsafe { ffi::sk_scene_get_size_w(self.ptr) }, unsafe {
            ffi::sk_scene_get_size_h(self.ptr)
        })
    }

    /// Sets a property exposed by `SKScene`.
    pub fn set_size(&self, size: CGSize) {
        unsafe { ffi::sk_scene_set_size(self.ptr, size.width, size.height) };
    }

    /// Returns a property exposed by `SKScene`.
    #[must_use]
    pub fn scale_mode(&self) -> SceneScaleMode {
        SceneScaleMode::from_raw(unsafe { ffi::sk_scene_get_scale_mode(self.ptr) })
    }

    /// Sets a property exposed by `SKScene`.
    pub fn set_scale_mode(&self, mode: SceneScaleMode) {
        unsafe { ffi::sk_scene_set_scale_mode(self.ptr, mode as i32) };
    }

    /// Sets a property exposed by `SKScene`.
    pub fn set_background_color(&self, color: crate::color::Color) {
        unsafe { ffi::sk_scene_set_background_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    /// Returns a property exposed by `SKScene`.
    #[must_use]
    pub fn camera(&self) -> Option<CameraNode> {
        unsafe { CameraNode::from_raw(ffi::sk_scene_get_camera(self.ptr)) }
    }

    /// Sets a property exposed by `SKScene`.
    pub fn set_camera(&self, camera: Option<&CameraNode>) {
        unsafe {
            ffi::sk_scene_set_camera(
                self.ptr,
                camera.map_or(core::ptr::null_mut(), CameraNode::as_ptr),
            );
        };
    }

    /// Sets a property exposed by `SKScene`.
    pub fn set_delegate(&self, delegate: Option<&SceneDelegate>) {
        unsafe {
            ffi::sk_scene_set_delegate(
                self.ptr,
                delegate.map_or(core::ptr::null_mut(), SceneDelegate::as_ptr),
            );
        };
    }

    /// Returns a property exposed by `SKScene`.
    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::sk_scene_has_delegate(self.ptr) }
    }

    /// Returns a property exposed by `SKScene`.
    #[must_use]
    pub fn anchor_point(&self) -> CGPoint {
        CGPoint::new(unsafe { ffi::sk_scene_get_anchor_x(self.ptr) }, unsafe {
            ffi::sk_scene_get_anchor_y(self.ptr)
        })
    }

    /// Sets a property exposed by `SKScene`.
    pub fn set_anchor_point(&self, anchor: CGPoint) {
        unsafe { ffi::sk_scene_set_anchor_point(self.ptr, anchor.x, anchor.y) };
    }

    /// Wraps `SKScene`.
    #[must_use]
    pub fn physics_world(&self) -> PhysicsWorld {
        unsafe { PhysicsWorld::from_raw_unchecked(ffi::sk_scene_physics_world(self.ptr)) }
    }

    /// Returns a property exposed by `SKScene`.
    #[must_use]
    pub fn view(&self) -> Option<View> {
        unsafe { View::from_raw(ffi::sk_scene_get_view(self.ptr)) }
    }

    /// Wraps `SKScene`.
    #[must_use]
    pub fn convert_point_from_view(&self, point: CGPoint) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_scene_convert_point_from_view_x(self.ptr, point.x, point.y) },
            unsafe { ffi::sk_scene_convert_point_from_view_y(self.ptr, point.x, point.y) },
        )
    }

    /// Wraps `SKScene`.
    #[must_use]
    pub fn convert_point_to_view(&self, point: CGPoint) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_scene_convert_point_to_view_x(self.ptr, point.x, point.y) },
            unsafe { ffi::sk_scene_convert_point_to_view_y(self.ptr, point.x, point.y) },
        )
    }
}
