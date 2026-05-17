use core::ffi::c_void;

use apple_cf::cg::{CGPoint, CGSize, CGVector};

use crate::ffi;
use crate::private::handle_type;
use crate::texture::Texture;
use crate::warp::AsWarpGeometry;

handle_type!(Action);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ActionTimingMode {
    #[default]
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

impl ActionTimingMode {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::EaseIn,
            2 => Self::EaseOut,
            3 => Self::EaseInEaseOut,
            _ => Self::Linear,
        }
    }
}

impl Action {
    #[must_use]
    pub fn move_by(dx: f64, dy: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_move_by(dx, dy, duration)) }
    }

    #[must_use]
    pub fn move_to(position: CGPoint, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_move_to(position.x, position.y, duration)) }
    }

    #[must_use]
    pub fn rotate_by(angle: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_rotate_by(angle, duration)) }
    }

    #[must_use]
    pub fn rotate_to(angle: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_rotate_to(angle, duration)) }
    }

    #[must_use]
    pub fn scale_by(scale: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_scale_by(scale, duration)) }
    }

    #[must_use]
    pub fn scale_to(scale: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_scale_to(scale, duration)) }
    }

    #[must_use]
    pub fn resize_to(width: f64, height: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_resize_to(width, height, duration)) }
    }

    #[must_use]
    pub fn scale_to_size(size: CGSize, duration: f64) -> Option<Self> {
        Self::resize_to(size.width, size.height, duration)
    }

    #[must_use]
    pub fn fade_in(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_fade_in(duration)) }
    }

    #[must_use]
    pub fn fade_out(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_fade_out(duration)) }
    }

    #[must_use]
    pub fn fade_to(alpha: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_fade_to(alpha, duration)) }
    }

    #[must_use]
    pub fn fade_alpha_by(alpha: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_fade_alpha_by(alpha, duration)) }
    }

    #[must_use]
    pub fn hide() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_hide()) }
    }

    #[must_use]
    pub fn unhide() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_unhide()) }
    }

    #[must_use]
    pub fn set_texture(texture: &Texture) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_set_texture(texture.as_ptr())) }
    }

    #[must_use]
    pub fn animate_with_textures(textures: &[&Texture], time_per_frame: f64) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = textures.iter().map(|texture| texture.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe {
            Self::from_raw(ffi::sk_action_animate_with_textures(
                raw_ptr,
                raw.len(),
                time_per_frame,
            ))
        }
    }

    #[must_use]
    pub fn wait(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_wait(duration)) }
    }

    #[must_use]
    pub fn sequence(actions: &[&Self]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = actions.iter().map(|action| action.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe { Self::from_raw(ffi::sk_action_sequence(raw_ptr, raw.len())) }
    }

    #[must_use]
    pub fn group(actions: &[&Self]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = actions.iter().map(|action| action.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            core::ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe { Self::from_raw(ffi::sk_action_group(raw_ptr, raw.len())) }
    }

    #[must_use]
    pub fn repeat_count(action: &Self, count: usize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_repeat(action.as_ptr(), count)) }
    }

    #[must_use]
    pub fn repeat_forever(action: &Self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_repeat_forever(action.as_ptr())) }
    }

    #[must_use]
    pub fn change_charge_to(value: f32, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_change_charge_to(value, duration)) }
    }

    #[must_use]
    pub fn apply_force(force: CGVector, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_apply_force(force.dx, force.dy, duration)) }
    }

    #[must_use]
    pub fn play() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_play()) }
    }

    #[must_use]
    pub fn change_volume_to(value: f32, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_change_volume_to(value, duration)) }
    }

    #[must_use]
    pub fn stereo_pan_to(value: f32, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_stereo_pan_to(value, duration)) }
    }

    #[must_use]
    pub fn warp_to<W: AsWarpGeometry>(warp: &W, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_warp_to(warp.as_warp_geometry_ptr(), duration)) }
    }

    #[must_use]
    pub fn animate_with_warps<W: AsWarpGeometry>(warps: &[&W], times: &[f64]) -> Option<Self> {
        if warps.len() != times.len() {
            return None;
        }
        let mut raw_warps: Vec<*mut c_void> = warps.iter().map(|warp| warp.as_warp_geometry_ptr()).collect();
        let warps_ptr = if raw_warps.is_empty() {
            core::ptr::null_mut()
        } else {
            raw_warps.as_mut_ptr().cast()
        };
        unsafe { Self::from_raw(ffi::sk_action_animate_with_warps(warps_ptr, times.as_ptr(), warps.len())) }
    }

    #[must_use]
    pub fn duration(&self) -> f64 {
        unsafe { ffi::sk_action_get_duration(self.ptr) }
    }

    pub fn set_duration(&self, duration: f64) {
        unsafe { ffi::sk_action_set_duration(self.ptr, duration) };
    }

    #[must_use]
    pub fn timing_mode(&self) -> ActionTimingMode {
        ActionTimingMode::from_raw(unsafe { ffi::sk_action_get_timing_mode(self.ptr) })
    }

    pub fn set_timing_mode(&self, mode: ActionTimingMode) {
        unsafe { ffi::sk_action_set_timing_mode(self.ptr, mode as i32) };
    }

    #[must_use]
    pub fn speed(&self) -> f64 {
        unsafe { ffi::sk_action_get_speed(self.ptr) }
    }

    pub fn set_speed(&self, speed: f64) {
        unsafe { ffi::sk_action_set_speed(self.ptr, speed) };
    }

    #[must_use]
    pub fn reversed(&self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_reversed(self.ptr)) }
    }
}
