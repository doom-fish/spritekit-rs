use core::ffi::c_void;

use apple_cf::cg::CGPoint;

use crate::ffi;
use crate::private::handle_type;
use crate::texture::Texture;

handle_type!(Action);

impl Action {
    /// Moves a node by `(dx, dy)` over `duration` seconds.
    #[must_use]
    pub fn move_by(dx: f64, dy: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_move_by(dx, dy, duration)) }
    }

    /// Moves a node to `position` over `duration` seconds.
    #[must_use]
    pub fn move_to(position: CGPoint, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_move_to(position.x, position.y, duration)) }
    }

    /// Rotates a node by `angle` radians over `duration` seconds.
    #[must_use]
    pub fn rotate_by(angle: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_rotate_by(angle, duration)) }
    }

    /// Rotates a node to `angle` radians over `duration` seconds.
    #[must_use]
    pub fn rotate_to(angle: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_rotate_to(angle, duration)) }
    }

    /// Scales a node by `scale` over `duration` seconds.
    #[must_use]
    pub fn scale_by(scale: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_scale_by(scale, duration)) }
    }

    /// Scales a node to `scale` over `duration` seconds.
    #[must_use]
    pub fn scale_to(scale: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_scale_to(scale, duration)) }
    }

    /// Resizes a sprite node to `(width, height)` over `duration` seconds.
    #[must_use]
    pub fn resize_to(width: f64, height: f64, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_resize_to(width, height, duration)) }
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

    /// Animates through a slice of textures.
    #[must_use]
    pub fn animate_with_textures(textures: &[&Texture], time_per_frame: f64) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = textures.iter().map(|t| t.as_ptr()).collect();
        unsafe {
            Self::from_raw(ffi::sk_action_animate_with_textures(
                raw.as_mut_ptr().cast(),
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
        let mut raw: Vec<*mut c_void> = actions.iter().map(|a| a.as_ptr()).collect();
        unsafe { Self::from_raw(ffi::sk_action_sequence(raw.as_mut_ptr().cast(), raw.len())) }
    }

    #[must_use]
    pub fn group(actions: &[&Self]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = actions.iter().map(|a| a.as_ptr()).collect();
        unsafe { Self::from_raw(ffi::sk_action_group(raw.as_mut_ptr().cast(), raw.len())) }
    }

    #[must_use]
    pub fn repeat_count(action: &Self, count: usize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_repeat(action.as_ptr(), count)) }
    }

    #[must_use]
    pub fn repeat_forever(action: &Self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_action_repeat_forever(action.as_ptr())) }
    }
}
