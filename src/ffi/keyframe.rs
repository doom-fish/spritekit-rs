use core::ffi::c_void;

extern "C" {
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_new_with_capacity(capacity: usize) -> *mut c_void;
    /// Returns a property exposed by `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_count(sequence: *mut c_void) -> usize;
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_add_scalar(sequence: *mut c_void, value: f64, time: f64);
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_remove_last(sequence: *mut c_void);
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_remove_at(sequence: *mut c_void, index: usize);
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_set_scalar(sequence: *mut c_void, value: f64, index: usize);
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_set_time(sequence: *mut c_void, time: f64, index: usize);
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_sample_scalar(
        sequence: *mut c_void,
        time: f64,
        out_value: *mut f64,
    ) -> bool;
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_get_interpolation_mode(sequence: *mut c_void) -> i32;
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_set_interpolation_mode(sequence: *mut c_void, mode: i32);
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_get_repeat_mode(sequence: *mut c_void) -> i32;
    /// Wraps `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
    pub fn sk_keyframe_sequence_set_repeat_mode(sequence: *mut c_void, mode: i32);
}
