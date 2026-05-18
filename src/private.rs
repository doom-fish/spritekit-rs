use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

macro_rules! handle_type {
    ($name:ident) => {
        /// Wrapper type for retained `SpriteKit` wrapper handles.
        pub struct $name {
            /// Field used by retained `SpriteKit` wrapper handles.
            pub(crate) ptr: *mut core::ffi::c_void,
            owned: bool,
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ptr", &self.ptr)
                    .field("owned", &self.owned)
                    .finish()
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.owned && !self.ptr.is_null() {
                    // SAFETY: The ptr is guaranteed to be a valid SpriteKit object reference
                    // that was created by from_raw(). Since owned=true, we own the reference
                    // and it's safe to release. The Swift bridge's sk_release is thread-safe.
                    unsafe { crate::ffi::sk_release(self.ptr) };
                    self.ptr = core::ptr::null_mut();
                }
            }
        }

        #[allow(dead_code)]
        impl $name {
            /// Converts a raw value from retained `SpriteKit` wrapper handles.
            ///
            /// # Safety
            ///
            /// Follow the safety contract required by the underlying retained `SpriteKit` wrapper handles API.
            pub(crate) unsafe fn from_raw(ptr: *mut core::ffi::c_void) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr, owned: true })
                }
            }

            /// Converts a raw value from retained `SpriteKit` wrapper handles.
            ///
            /// # Safety
            ///
            /// Follow the safety contract required by the underlying retained `SpriteKit` wrapper handles API.
            pub(crate) const unsafe fn from_raw_unchecked(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: true }
            }

            /// Converts a raw value from retained `SpriteKit` wrapper handles.
            ///
            /// # Safety
            ///
            /// Follow the safety contract required by the underlying retained `SpriteKit` wrapper handles API.
            pub(crate) const unsafe fn from_raw_borrowed(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: false }
            }

            /// Returns the raw pointer used by retained `SpriteKit` wrapper handles.
            #[must_use]
            pub const fn as_ptr(&self) -> *mut core::ffi::c_void {
                self.ptr
            }
        }
    };
}

/// Re-export used by retained `SpriteKit` wrapper handles.
pub(crate) use handle_type;

/// Wraps retained `SpriteKit` wrapper handles.
pub fn cstring_from_str(value: &str) -> Option<CString> {
    CString::new(value).ok()
}

/// Wraps retained `SpriteKit` wrapper handles.
#[allow(dead_code)]
pub fn cstring_from_path(path: &Path) -> Option<CString> {
    CString::new(path.as_os_str().as_bytes()).ok()
}
