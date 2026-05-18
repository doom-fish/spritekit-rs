use core::ffi::{c_char, c_void};

extern "C" {
    /// Wraps `SKAttribute`, `SKAttributeType`, and `SKAttributeValue`.
    pub fn sk_attribute_new(name: *const c_char, attribute_type: i32) -> *mut c_void;
    /// Wraps `SKAttribute`, `SKAttributeType`, and `SKAttributeValue`.
    pub fn sk_attribute_copy_name(attribute: *mut c_void) -> *mut c_char;
    /// Wraps `SKAttribute`, `SKAttributeType`, and `SKAttributeValue`.
    pub fn sk_attribute_get_type(attribute: *mut c_void) -> i32;

    /// Wraps `SKAttribute`, `SKAttributeType`, and `SKAttributeValue`.
    pub fn sk_attribute_value_new_with_float(value: f32) -> *mut c_void;
    /// Wraps `SKAttribute`, `SKAttributeType`, and `SKAttributeValue`.
    pub fn sk_attribute_value_get_float(value: *mut c_void) -> f32;
    /// Wraps `SKAttribute`, `SKAttributeType`, and `SKAttributeValue`.
    pub fn sk_attribute_value_set_float(value: *mut c_void, new_value: f32);
}
