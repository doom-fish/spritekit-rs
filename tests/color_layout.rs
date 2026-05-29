//! ABI layout assertions for the `#[repr(C)]` `Color` shared with the Swift bridge.
//!
//! `Color` is passed by value across the Rust <-> Swift `@_cdecl` FFI boundary
//! (its four `f32` channels are forwarded as `Float` arguments). If its size or
//! alignment ever drifts, colour data is silently corrupted. This test pins the
//! layout so accidental field reordering / type changes are caught at
//! `cargo test` time rather than as runtime garbage.

use std::mem::{align_of, size_of};

use spritekit::color::verify_ffi_layout;
use spritekit::Color;

#[test]
fn color_layout() {
    assert_eq!(size_of::<Color>(), 16, "Color size drifted");
    assert_eq!(align_of::<Color>(), 4, "Color alignment drifted");
}

#[test]
fn ffi_layout_verified() {
    assert!(
        verify_ffi_layout(),
        "Color layout disagrees with the pinned ABI"
    );
}
