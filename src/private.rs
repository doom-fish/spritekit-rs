use core::ffi::c_void;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use doom_fish_utils::callback_context::CallbackContext;

pub unsafe fn invoke_callback<T: Send + Sync + 'static, R>(
    context: *mut c_void,
    site: &str,
    f: impl FnOnce(&T) -> R,
) -> Option<R> {
    if context.is_null() {
        return None;
    }
    unsafe { CallbackContext::<T>::RETAIN(context) };
    let result = unsafe { CallbackContext::<T>::with(context, site, f) };
    unsafe { CallbackContext::<T>::RELEASE(context) };
    result
}

pub struct DelegateObject<T: Send + Sync + 'static> {
    ptr: *mut c_void,
    context: Option<CallbackContext<T>>,
}

impl<T: Send + Sync + 'static> DelegateObject<T> {
    pub fn new(value: T, create: impl FnOnce(*mut c_void) -> *mut c_void) -> Option<Self> {
        let context = CallbackContext::new(value);
        let retained = context.retained_ptr();
        let ptr = create(retained);
        if ptr.is_null() {
            unsafe { CallbackContext::<T>::RELEASE(retained) };
            return None;
        }
        Some(Self {
            ptr,
            context: Some(context),
        })
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn is_active(&self) -> bool {
        self.context
            .as_ref()
            .is_some_and(CallbackContext::is_active)
    }
}

impl<T: Send + Sync + 'static> Drop for DelegateObject<T> {
    fn drop(&mut self) {
        if let Some(context) = &self.context {
            context.deactivate();
        }
        if !self.ptr.is_null() {
            unsafe { crate::ffi::sk_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl<T: Send + Sync + 'static> core::fmt::Debug for DelegateObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DelegateObject")
            .field("ptr", &self.ptr)
            .field("owns_callbacks", &self.context.is_some())
            .field("active", &self.is_active())
            .finish()
    }
}

pub fn is_main_thread() -> bool {
    unsafe { libc::pthread_main_np() != 0 }
}

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
