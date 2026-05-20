use std::error::Error;
use std::fmt;

/// Wrapper type for `SpriteKit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteKitError {
    message: String,
}

impl SpriteKitError {
    /// Wraps `SpriteKit`.
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

/// Wraps `SpriteKit` bridge failures.
///
/// # Safety
///
/// Follow the safety contract required by the underlying `SpriteKit` bridge failures API.
pub(crate) unsafe fn take_string(ptr: *mut libc::c_char) -> Option<String> {
    doom_fish_utils::ffi_string::take_owned_cstring_c(ptr, |p| libc::free(p.cast()))
}
