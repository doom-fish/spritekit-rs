use crate::ffi;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Attribute);
handle_type!(AttributeValue);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum AttributeType {
    #[default]
    None = 0,
    Float = 1,
    VectorFloat2 = 2,
    VectorFloat3 = 3,
    VectorFloat4 = 4,
    HalfFloat = 5,
    VectorHalfFloat2 = 6,
    VectorHalfFloat3 = 7,
    VectorHalfFloat4 = 8,
}

impl AttributeType {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Float,
            2 => Self::VectorFloat2,
            3 => Self::VectorFloat3,
            4 => Self::VectorFloat4,
            5 => Self::HalfFloat,
            6 => Self::VectorHalfFloat2,
            7 => Self::VectorHalfFloat3,
            8 => Self::VectorHalfFloat4,
            _ => Self::None,
        }
    }
}

impl Attribute {
    #[must_use]
    pub fn new(name: &str, attribute_type: AttributeType) -> Option<Self> {
        let name = cstring_from_str(name)?;
        unsafe { Self::from_raw(ffi::sk_attribute_new(name.as_ptr(), attribute_type as i32)) }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_attribute_copy_name(self.ptr)) }
    }

    #[must_use]
    pub fn attribute_type(&self) -> AttributeType {
        AttributeType::from_raw(unsafe { ffi::sk_attribute_get_type(self.ptr) })
    }
}

impl AttributeValue {
    #[must_use]
    pub fn with_float(value: f32) -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_attribute_value_new_with_float(value)) }
    }

    #[must_use]
    pub fn float_value(&self) -> f32 {
        unsafe { ffi::sk_attribute_value_get_float(self.ptr) }
    }

    pub fn set_float_value(&self, value: f32) {
        unsafe { ffi::sk_attribute_value_set_float(self.ptr, value) };
    }
}
