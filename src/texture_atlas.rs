use core::{
    ffi::{c_char, c_void},
    mem::ManuallyDrop,
    ptr, slice,
};
use std::ffi::CString;
use std::sync::Mutex;
#[cfg(feature = "async")]
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, MutexGuard},
    task::{Context, Poll, Waker},
};

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::panic_safe::catch_user_panic;

use crate::error::SpriteKitError;
use crate::ffi;
use crate::private::{cstring_from_str, handle_type};
use crate::texture::Texture;

handle_type!(TextureAtlas);

type TextureAtlasCompletionCallback = Box<dyn FnOnce() + Send + 'static>;
type TextureAtlasNamedCompletionCallback =
    Box<dyn FnOnce(Result<Vec<TextureAtlas>, SpriteKitError>) + Send + 'static>;

#[cfg(feature = "async")]
type TextureAtlasNamedAsyncCompletionCallback =
    Box<dyn FnOnce(Result<Vec<OwnedTextureAtlasHandle>, SpriteKitError>) + Send + 'static>;

type ReleaseContext = unsafe extern "C" fn(*mut c_void);

#[cfg(feature = "async")]
struct OwnedTextureAtlasHandle(usize);

#[cfg(feature = "async")]
impl OwnedTextureAtlasHandle {
    fn from_raw(handle: *mut c_void) -> Option<Self> {
        (!handle.is_null()).then_some(Self(handle as usize))
    }

    fn into_texture_atlas(mut self) -> TextureAtlas {
        let handle = self.0 as *mut c_void;
        self.0 = 0;
        unsafe { TextureAtlas::from_raw_unchecked(handle) }
    }
}

#[cfg(feature = "async")]
impl Drop for OwnedTextureAtlasHandle {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe { ffi::sk_release(self.0 as *mut c_void) };
            self.0 = 0;
        }
    }
}

fn one_shot_context<T: Send + 'static>(callback: T) -> (*mut c_void, ReleaseContext) {
    let context = ManuallyDrop::new(CallbackContext::new(Mutex::new(Some(callback))));
    (
        context.as_ptr(),
        CallbackContext::<Mutex<Option<T>>>::RELEASE,
    )
}

unsafe fn take_one_shot<T: Send + 'static>(context: *mut c_void, site: &str) -> Option<T> {
    unsafe {
        CallbackContext::<Mutex<Option<T>>>::with(context, site, |slot| {
            slot.lock().ok().and_then(|mut slot| slot.take())
        })
    }
    .flatten()
}

unsafe extern "C" fn texture_atlas_completion(context: *mut c_void) {
    const SITE: &str = "spritekit::TextureAtlas::preload_completion";
    if let Some(callback) =
        unsafe { take_one_shot::<TextureAtlasCompletionCallback>(context, SITE) }
    {
        catch_user_panic(SITE, callback);
    }
}

unsafe extern "C" fn texture_atlas_named_completion(
    context: *mut c_void,
    error: *mut c_char,
    atlas_handles: *mut *mut c_void,
    atlas_count: usize,
) {
    const SITE: &str = "spritekit::TextureAtlas::preload_named_completion";
    let atlases = take_texture_atlases(atlas_handles, atlas_count);
    let result = match unsafe { crate::error::take_string(error) } {
        Some(message) => {
            drop(atlases);
            Err(SpriteKitError::new(message))
        }
        None => Ok(atlases),
    };
    if let Some(callback) =
        unsafe { take_one_shot::<TextureAtlasNamedCompletionCallback>(context, SITE) }
    {
        catch_user_panic(SITE, || callback(result));
    }
}

#[cfg(feature = "async")]
unsafe extern "C" fn texture_atlas_named_async_completion(
    context: *mut c_void,
    error: *mut c_char,
    atlas_handles: *mut *mut c_void,
    atlas_count: usize,
) {
    const SITE: &str = "spritekit::TextureAtlas::preload_named_async_completion";
    let atlases = take_owned_texture_atlas_handles(atlas_handles, atlas_count);
    let result = match unsafe { crate::error::take_string(error) } {
        Some(message) => {
            drop(atlases);
            Err(SpriteKitError::new(message))
        }
        None => Ok(atlases),
    };
    if let Some(callback) =
        unsafe { take_one_shot::<TextureAtlasNamedAsyncCompletionCallback>(context, SITE) }
    {
        catch_user_panic(SITE, || callback(result));
    }
}

fn texture_atlas_completion_context<F>(completion: F) -> (*mut c_void, ReleaseContext)
where
    F: FnOnce() + Send + 'static,
{
    let callback: TextureAtlasCompletionCallback = Box::new(completion);
    one_shot_context(callback)
}

fn texture_atlas_named_completion_context<F>(completion: F) -> (*mut c_void, ReleaseContext)
where
    F: FnOnce(Result<Vec<TextureAtlas>, SpriteKitError>) + Send + 'static,
{
    let callback: TextureAtlasNamedCompletionCallback = Box::new(completion);
    one_shot_context(callback)
}

#[cfg(feature = "async")]
fn texture_atlas_named_async_completion_context<F>(completion: F) -> (*mut c_void, ReleaseContext)
where
    F: FnOnce(Result<Vec<OwnedTextureAtlasHandle>, SpriteKitError>) + Send + 'static,
{
    let callback: TextureAtlasNamedAsyncCompletionCallback = Box::new(completion);
    one_shot_context(callback)
}

fn texture_atlas_name_cstrings(atlas_names: &[&str]) -> Result<Vec<CString>, SpriteKitError> {
    atlas_names
        .iter()
        .map(|name| {
            cstring_from_str(name)
                .ok_or_else(|| SpriteKitError::new("atlas name contained an interior NUL byte"))
        })
        .collect()
}

fn take_texture_atlases(atlas_handles: *mut *mut c_void, atlas_count: usize) -> Vec<TextureAtlas> {
    if atlas_handles.is_null() || atlas_count == 0 {
        return Vec::new();
    }

    let atlas_handles = unsafe { slice::from_raw_parts(atlas_handles.cast_const(), atlas_count) };
    atlas_handles
        .iter()
        .filter_map(|&handle| unsafe { TextureAtlas::from_raw(handle) })
        .collect()
}

#[cfg(feature = "async")]
fn take_owned_texture_atlas_handles(
    atlas_handles: *mut *mut c_void,
    atlas_count: usize,
) -> Vec<OwnedTextureAtlasHandle> {
    if atlas_handles.is_null() || atlas_count == 0 {
        return Vec::new();
    }

    let atlas_handles = unsafe { slice::from_raw_parts(atlas_handles.cast_const(), atlas_count) };
    atlas_handles
        .iter()
        .filter_map(|&handle| OwnedTextureAtlasHandle::from_raw(handle))
        .collect()
}

#[cfg(feature = "async")]
struct CallbackFuture<T> {
    state: Arc<Mutex<CallbackFutureState<T>>>,
}

#[cfg(feature = "async")]
struct CallbackFutureState<T> {
    value: Option<Result<T, SpriteKitError>>,
    waker: Option<Waker>,
}

#[cfg(feature = "async")]
struct CallbackFutureCompleter<T> {
    state: Option<Arc<Mutex<CallbackFutureState<T>>>>,
}

#[cfg(feature = "async")]
fn lock_callback_future_state<T>(
    state: &Mutex<CallbackFutureState<T>>,
) -> MutexGuard<'_, CallbackFutureState<T>> {
    match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

#[cfg(feature = "async")]
impl<T> CallbackFuture<T> {
    fn new() -> (Self, CallbackFutureCompleter<T>) {
        let state = Arc::new(Mutex::new(CallbackFutureState {
            value: None,
            waker: None,
        }));
        (
            Self {
                state: Arc::clone(&state),
            },
            CallbackFutureCompleter { state: Some(state) },
        )
    }
}

#[cfg(feature = "async")]
impl<T> Future for CallbackFuture<T> {
    type Output = Result<T, SpriteKitError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = lock_callback_future_state(&self.state);
        state.value.take().map_or_else(
            || {
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            },
            Poll::Ready,
        )
    }
}

#[cfg(feature = "async")]
impl<T> CallbackFutureCompleter<T> {
    fn resolve(&mut self, value: Result<T, SpriteKitError>) {
        let Some(state) = self.state.take() else {
            return;
        };
        let waker = {
            let mut state = lock_callback_future_state(&state);
            state.value = Some(value);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

#[cfg(feature = "async")]
impl<T> Drop for CallbackFutureCompleter<T> {
    fn drop(&mut self) {
        self.resolve(Err(SpriteKitError::new(
            "SpriteKit released the completion handler without calling it",
        )));
    }
}

impl TextureAtlas {
    /// Wraps `SKTextureAtlas`.
    #[must_use]
    pub fn named(name: &str) -> Option<Self> {
        let name = cstring_from_str(name)?;
        unsafe { Self::from_raw(ffi::sk_texture_atlas_named(name.as_ptr())) }
    }

    /// Returns a property exposed by `SKTextureAtlas`.
    #[must_use]
    pub fn texture_names_count(&self) -> usize {
        unsafe { ffi::sk_texture_atlas_get_texture_names_count(self.ptr) }
    }

    /// Wraps `SKTextureAtlas`.
    #[must_use]
    pub fn texture_named(&self, name: &str) -> Option<Texture> {
        let name = cstring_from_str(name)?;
        unsafe { Texture::from_raw(ffi::sk_texture_atlas_texture_named(self.ptr, name.as_ptr())) }
    }

    /// Wraps `SKTextureAtlas`.
    ///
    /// `SpriteKit` invokes the completion handler on an arbitrary queue after the atlas finishes
    /// preloading.
    pub fn preload_with_completion_handler<F>(&self, completion: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (context, release) = texture_atlas_completion_context(completion);
        unsafe {
            ffi::sk_texture_atlas_preload(
                self.ptr,
                context,
                Some(texture_atlas_completion),
                Some(release),
            );
        };
    }

    /// Wraps `SKTextureAtlas`.
    ///
    /// `SpriteKit` invokes the completion handler on an arbitrary queue after all atlases finish
    /// preloading.
    pub fn preload_texture_atlases_with_completion_handler<F>(atlases: &[&Self], completion: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut raw: Vec<*mut c_void> = atlases.iter().map(|atlas| atlas.as_ptr()).collect();
        let raw_ptr = if raw.is_empty() {
            ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        let (context, release) = texture_atlas_completion_context(completion);
        unsafe {
            ffi::sk_texture_atlas_preload_texture_atlases(
                raw_ptr,
                raw.len(),
                context,
                Some(texture_atlas_completion),
                Some(release),
            );
        };
    }

    /// Wraps `SKTextureAtlas`.
    ///
    /// `SpriteKit` invokes the completion handler on an arbitrary queue with either preloaded atlases
    /// or a preload error.
    pub fn preload_texture_atlases_named_with_completion_handler<F>(
        atlas_names: &[&str],
        completion: F,
    ) -> Result<(), SpriteKitError>
    where
        F: FnOnce(Result<Vec<Self>, SpriteKitError>) + Send + 'static,
    {
        let atlas_names = texture_atlas_name_cstrings(atlas_names)?;
        let mut raw_names: Vec<*const c_char> =
            atlas_names.iter().map(|name| name.as_ptr()).collect();
        let raw_ptr = if raw_names.is_empty() {
            ptr::null_mut()
        } else {
            raw_names.as_mut_ptr().cast()
        };
        let (context, release) = texture_atlas_named_completion_context(completion);
        unsafe {
            ffi::sk_texture_atlas_preload_texture_atlases_named(
                raw_ptr,
                raw_names.len(),
                context,
                Some(texture_atlas_named_completion),
                Some(release),
            );
        };
        Ok(())
    }

    /// Async wrapper for `preloadWithCompletionHandler:`.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn preload_async(&self) -> impl Future<Output = Result<(), SpriteKitError>> + Send {
        let (future, mut completer) = CallbackFuture::new();
        self.preload_with_completion_handler(move || completer.resolve(Ok(())));
        future
    }

    /// Async wrapper for `preloadTextureAtlases(_:withCompletionHandler:)`.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn preload_texture_atlases_async(
        atlases: &[&Self],
    ) -> impl Future<Output = Result<(), SpriteKitError>> + Send {
        let (future, mut completer) = CallbackFuture::new();
        Self::preload_texture_atlases_with_completion_handler(atlases, move || {
            completer.resolve(Ok(()));
        });
        future
    }

    /// Async wrapper for `preloadTextureAtlasesNamed(_:withCompletionHandler:)`.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn preload_texture_atlases_named_async(
        atlas_names: &[&str],
    ) -> impl Future<Output = Result<Vec<Self>, SpriteKitError>> + Send {
        let scheduled = {
            let (future, mut completer) = CallbackFuture::new();
            texture_atlas_name_cstrings(atlas_names).map(|atlas_names| {
                let mut raw_names: Vec<*const c_char> =
                    atlas_names.iter().map(|name| name.as_ptr()).collect();
                let raw_ptr = if raw_names.is_empty() {
                    ptr::null_mut()
                } else {
                    raw_names.as_mut_ptr().cast()
                };
                let (context, release) =
                    texture_atlas_named_async_completion_context(move |result| {
                        completer.resolve(result);
                    });
                unsafe {
                    ffi::sk_texture_atlas_preload_texture_atlases_named(
                        raw_ptr,
                        raw_names.len(),
                        context,
                        Some(texture_atlas_named_async_completion),
                        Some(release),
                    );
                };
                future
            })
        };
        async move {
            match scheduled {
                Ok(future) => future.await.map(|atlases| {
                    atlases
                        .into_iter()
                        .map(OwnedTextureAtlasHandle::into_texture_atlas)
                        .collect()
                }),
                Err(error) => Err(error),
            }
        }
    }
}

#[cfg(all(test, feature = "async"))]
mod tests {
    use super::*;

    const DROPPED: &str = "SpriteKit released the completion handler without calling it";

    #[test]
    fn preload_future_fails_when_the_bridge_drops_the_completion() {
        let (future, mut completer) = CallbackFuture::<()>::new();
        let (context, release) =
            texture_atlas_completion_context(move || completer.resolve(Ok(())));
        unsafe {
            ffi::sk_texture_atlas_preload(
                ptr::null_mut(),
                context,
                Some(texture_atlas_completion),
                Some(release),
            );
        }
        let error = pollster::block_on(future).expect_err("the completion never ran");
        assert_eq!(error.to_string(), DROPPED);
    }

    #[test]
    fn atlases_preload_future_fails_when_spritekit_frees_a_completion_that_never_reached_rust() {
        let (future, mut completer) = CallbackFuture::<()>::new();
        let (context, release) =
            texture_atlas_completion_context(move || completer.resolve(Ok(())));
        unsafe {
            ffi::sk_texture_atlas_preload_texture_atlases(
                ptr::null_mut(),
                0,
                context,
                None,
                Some(release),
            );
        }
        let error = pollster::block_on(future).expect_err("the completion never ran");
        assert_eq!(error.to_string(), DROPPED);
    }

    #[test]
    fn named_preload_future_fails_when_spritekit_frees_a_completion_that_never_reached_rust() {
        let (future, mut completer) = CallbackFuture::<Vec<OwnedTextureAtlasHandle>>::new();
        let (context, release) =
            texture_atlas_named_async_completion_context(move |result| completer.resolve(result));
        unsafe {
            ffi::sk_texture_atlas_preload_texture_atlases_named(
                ptr::null_mut(),
                0,
                context,
                None,
                Some(release),
            );
        }
        let Err(error) = pollster::block_on(future) else {
            panic!("the completion never ran, so the future must fail");
        };
        assert_eq!(error.to_string(), DROPPED);
    }

    #[test]
    fn dropping_a_completer_after_it_resolved_keeps_the_value() {
        let (future, mut completer) = CallbackFuture::<u8>::new();
        completer.resolve(Ok(7));
        drop(completer);
        assert_eq!(pollster::block_on(future), Ok(7));
    }
}
