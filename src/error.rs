use std::error::Error;
use std::ffi::CStr;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteKitError {
    message: String,
}

impl SpriteKitError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for SpriteKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for SpriteKitError {}

pub(crate) unsafe fn take_string(ptr: *mut libc::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    libc::free(ptr.cast());
    Some(value)
}
