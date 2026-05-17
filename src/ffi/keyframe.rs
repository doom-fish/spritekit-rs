use core::ffi::c_void;

extern "C" {
    pub fn sk_keyframe_sequence_new_with_capacity(capacity: usize) -> *mut c_void;
    pub fn sk_keyframe_sequence_count(sequence: *mut c_void) -> usize;
    pub fn sk_keyframe_sequence_add_scalar(sequence: *mut c_void, value: f64, time: f64);
    pub fn sk_keyframe_sequence_remove_last(sequence: *mut c_void);
    pub fn sk_keyframe_sequence_remove_at(sequence: *mut c_void, index: usize);
    pub fn sk_keyframe_sequence_set_scalar(sequence: *mut c_void, value: f64, index: usize);
    pub fn sk_keyframe_sequence_set_time(sequence: *mut c_void, time: f64, index: usize);
    pub fn sk_keyframe_sequence_sample_scalar(
        sequence: *mut c_void,
        time: f64,
        out_value: *mut f64,
    ) -> bool;
    pub fn sk_keyframe_sequence_get_interpolation_mode(sequence: *mut c_void) -> i32;
    pub fn sk_keyframe_sequence_set_interpolation_mode(sequence: *mut c_void, mode: i32);
    pub fn sk_keyframe_sequence_get_repeat_mode(sequence: *mut c_void) -> i32;
    pub fn sk_keyframe_sequence_set_repeat_mode(sequence: *mut c_void, mode: i32);
}
