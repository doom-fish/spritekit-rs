use crate::ffi;
use crate::private::handle_type;

handle_type!(KeyframeSequence);

/// Enum for `SKInterpolationMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum InterpolationMode {
    #[default]
    Linear = 1,
    Spline = 2,
    Step = 3,
}

impl InterpolationMode {
    /// Converts a raw value from `SKInterpolationMode`.
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            2 => Self::Spline,
            3 => Self::Step,
            _ => Self::Linear,
        }
    }
}

/// Enum for `SKRepeatMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum RepeatMode {
    #[default]
    Clamp = 1,
    Loop = 2,
}

impl RepeatMode {
    /// Converts a raw value from `SKRepeatMode`.
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            2 => Self::Loop,
            _ => Self::Clamp,
        }
    }
}

impl KeyframeSequence {
    /// Wraps `SKKeyframeSequence`.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_keyframe_sequence_new_with_capacity(capacity)) }
    }

    /// Wraps `SKKeyframeSequence`.
    #[must_use]
    pub fn from_scalars(keyframes: &[(f64, f64)]) -> Option<Self> {
        let sequence = Self::with_capacity(keyframes.len())?;
        for &(time, value) in keyframes {
            sequence.add_scalar_keyframe(value, time);
        }
        Some(sequence)
    }

    /// Returns a property exposed by `SKKeyframeSequence`.
    #[must_use]
    pub fn count(&self) -> usize {
        unsafe { ffi::sk_keyframe_sequence_count(self.ptr) }
    }

    /// Wraps `SKKeyframeSequence`.
    pub fn add_scalar_keyframe(&self, value: f64, time: f64) {
        unsafe { ffi::sk_keyframe_sequence_add_scalar(self.ptr, value, time) };
    }

    /// Wraps `SKKeyframeSequence`.
    pub fn remove_last_keyframe(&self) {
        unsafe { ffi::sk_keyframe_sequence_remove_last(self.ptr) };
    }

    /// Wraps `SKKeyframeSequence`.
    pub fn remove_keyframe_at(&self, index: usize) {
        unsafe { ffi::sk_keyframe_sequence_remove_at(self.ptr, index) };
    }

    /// Sets a property exposed by `SKKeyframeSequence`.
    pub fn set_scalar_keyframe_value(&self, index: usize, value: f64) {
        unsafe { ffi::sk_keyframe_sequence_set_scalar(self.ptr, value, index) };
    }

    /// Sets a property exposed by `SKKeyframeSequence`.
    pub fn set_keyframe_time(&self, index: usize, time: f64) {
        unsafe { ffi::sk_keyframe_sequence_set_time(self.ptr, time, index) };
    }

    /// Wraps `SKKeyframeSequence`.
    #[must_use]
    pub fn sample_scalar(&self, time: f64) -> Option<f64> {
        let mut value = 0.0;
        let ok = unsafe { ffi::sk_keyframe_sequence_sample_scalar(self.ptr, time, &mut value) };
        ok.then_some(value)
    }

    /// Wraps `SKKeyframeSequence`.
    #[must_use]
    pub fn interpolation_mode(&self) -> InterpolationMode {
        InterpolationMode::from_raw(unsafe {
            ffi::sk_keyframe_sequence_get_interpolation_mode(self.ptr)
        })
    }

    /// Sets a property exposed by `SKKeyframeSequence`.
    pub fn set_interpolation_mode(&self, mode: InterpolationMode) {
        unsafe { ffi::sk_keyframe_sequence_set_interpolation_mode(self.ptr, mode as i32) };
    }

    /// Wraps `SKKeyframeSequence`.
    #[must_use]
    pub fn repeat_mode(&self) -> RepeatMode {
        RepeatMode::from_raw(unsafe { ffi::sk_keyframe_sequence_get_repeat_mode(self.ptr) })
    }

    /// Sets a property exposed by `SKKeyframeSequence`.
    pub fn set_repeat_mode(&self, mode: RepeatMode) {
        unsafe { ffi::sk_keyframe_sequence_set_repeat_mode(self.ptr, mode as i32) };
    }
}
