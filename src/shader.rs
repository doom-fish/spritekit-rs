use crate::ffi;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Shader);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum UniformType {
    #[default]
    None = 0,
    Float = 1,
    FloatVector2 = 2,
    FloatVector3 = 3,
    FloatVector4 = 4,
    FloatMatrix2 = 5,
    FloatMatrix3 = 6,
    FloatMatrix4 = 7,
    Texture = 8,
}

impl UniformType {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Float,
            2 => Self::FloatVector2,
            3 => Self::FloatVector3,
            4 => Self::FloatVector4,
            5 => Self::FloatMatrix2,
            6 => Self::FloatMatrix3,
            7 => Self::FloatMatrix4,
            8 => Self::Texture,
            _ => Self::None,
        }
    }
}

impl Shader {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::sk_shader_new()) }
    }

    #[must_use]
    pub fn with_source(source: &str) -> Option<Self> {
        let source = cstring_from_str(source)?;
        unsafe { Self::from_raw(ffi::sk_shader_new_with_source(source.as_ptr())) }
    }

    #[must_use]
    pub fn source(&self) -> Option<String> {
        unsafe { crate::error::take_string(ffi::sk_shader_copy_source(self.ptr)) }
    }

    pub fn set_source(&self, source: &str) {
        if let Some(source) = cstring_from_str(source) {
            unsafe { ffi::sk_shader_set_source(self.ptr, source.as_ptr()) };
        }
    }

    #[must_use]
    pub fn uniform_count(&self) -> usize {
        unsafe { ffi::sk_shader_uniform_count(self.ptr) }
    }

    pub fn add_float_uniform(&self, name: &str, value: f32) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::sk_shader_add_float_uniform(self.ptr, name.as_ptr(), value) };
        }
    }

    #[must_use]
    pub fn float_uniform_named(&self, name: &str) -> Option<f32> {
        let name = cstring_from_str(name)?;
        let mut value = 0.0;
        let ok = unsafe { ffi::sk_shader_get_float_uniform(self.ptr, name.as_ptr(), &mut value) };
        ok.then_some(value)
    }

    pub fn remove_uniform_named(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::sk_shader_remove_uniform_named(self.ptr, name.as_ptr()) };
        }
    }

    #[must_use]
    pub fn uniform_type_named(&self, name: &str) -> Option<UniformType> {
        let name = cstring_from_str(name)?;
        let mut value = 0;
        let ok = unsafe { ffi::sk_shader_get_uniform_type(self.ptr, name.as_ptr(), &mut value) };
        ok.then_some(UniformType::from_raw(value))
    }

    #[must_use]
    pub fn attribute_count(&self) -> usize {
        unsafe { ffi::sk_shader_attribute_count(self.ptr) }
    }
}

impl Default for Shader {
    fn default() -> Self {
        Self::new().expect("SKShader creation failed")
    }
}
