use core::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};

use apple_cf::cg::{CGPoint, CGSize};

use crate::camera_node::CameraNode;
use crate::ffi;
use crate::node::AsNode;
use crate::physics_world::PhysicsWorld;
use crate::private::handle_type;
use crate::view::View;

handle_type!(Scene);
handle_type!(SceneDelegate);

type SceneUpdateCallback = Box<dyn Fn(f64) + Send + Sync + 'static>;

struct SceneDelegateContext {
    update: Option<SceneUpdateCallback>,
}

extern "C" fn scene_delegate_update(context: *mut c_void, current_time: f64) {
    let context = unsafe { &*(context.cast::<SceneDelegateContext>()) };
    if let Some(callback) = &context.update {
        let _ = catch_unwind(AssertUnwindSafe(|| callback(current_time)));
    }
}

extern "C" fn scene_delegate_release(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<SceneDelegateContext>()));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SceneScaleMode {
    Fill = 0,
    AspectFill = 1,
    AspectFit = 2,
    ResizeFill = 3,
}

impl SceneScaleMode {
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
    #[must_use]
    pub fn new() -> Option<Self> {
        let context = Box::new(SceneDelegateContext { update: None });
        unsafe {
            Self::from_raw(ffi::sk_scene_delegate_new(
                Box::into_raw(context).cast(),
                Some(scene_delegate_update),
                Some(scene_delegate_release),
            ))
        }
    }

    #[must_use]
    pub fn with_update<F>(update: F) -> Option<Self>
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        let context = Box::new(SceneDelegateContext {
            update: Some(Box::new(update)),
        });
        unsafe {
            Self::from_raw(ffi::sk_scene_delegate_new(
                Box::into_raw(context).cast(),
                Some(scene_delegate_update),
                Some(scene_delegate_release),
            ))
        }
    }
}

impl Scene {
    #[must_use]
    pub fn with_size(size: CGSize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_scene_new_with_size(size.width, size.height)) }
    }

    #[must_use]
    pub fn size(&self) -> CGSize {
        CGSize::new(
            unsafe { ffi::sk_scene_get_size_w(self.ptr) },
            unsafe { ffi::sk_scene_get_size_h(self.ptr) },
        )
    }

    pub fn set_size(&self, size: CGSize) {
        unsafe { ffi::sk_scene_set_size(self.ptr, size.width, size.height) };
    }

    #[must_use]
    pub fn scale_mode(&self) -> SceneScaleMode {
        SceneScaleMode::from_raw(unsafe { ffi::sk_scene_get_scale_mode(self.ptr) })
    }

    pub fn set_scale_mode(&self, mode: SceneScaleMode) {
        unsafe { ffi::sk_scene_set_scale_mode(self.ptr, mode as i32) };
    }

    pub fn set_background_color(&self, color: crate::color::Color) {
        unsafe { ffi::sk_scene_set_background_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    #[must_use]
    pub fn camera(&self) -> Option<CameraNode> {
        unsafe { CameraNode::from_raw(ffi::sk_scene_get_camera(self.ptr)) }
    }

    pub fn set_camera(&self, camera: Option<&CameraNode>) {
        unsafe {
            ffi::sk_scene_set_camera(
                self.ptr,
                camera.map_or(core::ptr::null_mut(), CameraNode::as_ptr),
            );
        };
    }

    pub fn set_delegate(&self, delegate: Option<&SceneDelegate>) {
        unsafe {
            ffi::sk_scene_set_delegate(
                self.ptr,
                delegate.map_or(core::ptr::null_mut(), SceneDelegate::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::sk_scene_has_delegate(self.ptr) }
    }

    #[must_use]
    pub fn anchor_point(&self) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_scene_get_anchor_x(self.ptr) },
            unsafe { ffi::sk_scene_get_anchor_y(self.ptr) },
        )
    }

    pub fn set_anchor_point(&self, anchor: CGPoint) {
        unsafe { ffi::sk_scene_set_anchor_point(self.ptr, anchor.x, anchor.y) };
    }

    #[must_use]
    pub fn physics_world(&self) -> PhysicsWorld {
        unsafe { PhysicsWorld::from_raw_unchecked(ffi::sk_scene_physics_world(self.ptr)) }
    }

    #[must_use]
    pub fn view(&self) -> Option<View> {
        unsafe { View::from_raw(ffi::sk_scene_get_view(self.ptr)) }
    }

    #[must_use]
    pub fn convert_point_from_view(&self, point: CGPoint) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_scene_convert_point_from_view_x(self.ptr, point.x, point.y) },
            unsafe { ffi::sk_scene_convert_point_from_view_y(self.ptr, point.x, point.y) },
        )
    }

    #[must_use]
    pub fn convert_point_to_view(&self, point: CGPoint) -> CGPoint {
        CGPoint::new(
            unsafe { ffi::sk_scene_convert_point_to_view_x(self.ptr, point.x, point.y) },
            unsafe { ffi::sk_scene_convert_point_to_view_y(self.ptr, point.x, point.y) },
        )
    }
}
