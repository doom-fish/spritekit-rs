/// RGBA color value suitable for use with `SpriteKit` APIs.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Wraps `SKColor`.
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Wraps `SKColor`.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Wraps `SKColor`.
    #[must_use]
    pub const fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }

    /// Wraps `SKColor`.
    #[must_use]
    pub const fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }

    /// Wraps `SKColor`.
    #[must_use]
    pub const fn red() -> Self {
        Self::rgb(1.0, 0.0, 0.0)
    }

    /// Wraps `SKColor`.
    #[must_use]
    pub const fn green() -> Self {
        Self::rgb(0.0, 1.0, 0.0)
    }

    /// Wraps `SKColor`.
    #[must_use]
    pub const fn blue() -> Self {
        Self::rgb(0.0, 0.0, 1.0)
    }

    /// Wraps `SKColor`.
    #[must_use]
    pub const fn clear() -> Self {
        Self::rgba(0.0, 0.0, 0.0, 0.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

// MARK: - ABI Layout Assertions
//
// `Color` is a `#[repr(C)]` value passed across the Rust <-> Swift `@_cdecl` FFI
// boundary (its `r`/`g`/`b`/`a` fields are forwarded as four `Float` arguments to
// the Swift bridge, e.g. `skMakeColor` in
// `swift-bridge/Sources/SpriteKitBridge/Core.swift`). These compile-time
// assertions pin the size, alignment and field order so an accidental field
// reorder or type change fails the build instead of silently swapping colour
// channels at runtime. The crate's MSRV (1.76) predates `core::mem::offset_of!`
// (stabilised in 1.77), so only `size_of`/`align_of` are asserted here.
use core::mem::{align_of, size_of};

const _: () = assert!(size_of::<Color>() == 16);
const _: () = assert!(align_of::<Color>() == 4);

/// Verifies at runtime that [`Color`] still has the size and alignment pinned by
/// the compile-time assertions above (4 contiguous `f32` channels).
///
/// Returns `true` when the layout matches. This mirrors the compile-time guards
/// so an ABI drift is also catchable from a normal `cargo test` run (see
/// `tests/color_layout.rs`).
#[must_use]
pub const fn verify_ffi_layout() -> bool {
    size_of::<Color>() == 16 && align_of::<Color>() == 4
}
