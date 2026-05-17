use apple_cf::cg::CGRect;
use apple_metal::{CommandBuffer, MetalDevice, MetalTexture};

use crate::color::Color;
use crate::error::SpriteKitError;
use crate::ffi;
use crate::private::handle_type;
use crate::scene::Scene;

handle_type!(RenderPassDescriptor);
handle_type!(Renderer);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
}

impl RenderPassDescriptor {
    /// Creates a render-pass descriptor that clears the texture to `clear_color`.
    #[must_use]
    pub fn for_texture(texture: &MetalTexture, clear_color: Color) -> Option<Self> {
        Self::for_texture_with_actions(texture, clear_color, LoadAction::Clear, StoreAction::Store)
    }

    #[must_use]
    pub fn for_texture_with_actions(
        texture: &MetalTexture,
        clear_color: Color,
        load_action: LoadAction,
        store_action: StoreAction,
    ) -> Option<Self> {
        // SAFETY: sk_render_pass_descriptor_new_for_texture is a Swift FFI function that
        // returns a retained SpriteKit object pointer (or null). The FFI binding guarantees
        // it's safe to pass a valid MTLTexture pointer from apple-metal. from_raw() handles
        // the null case and sets up proper ownership semantics (owned=true for Drop).
        unsafe {
            Self::from_raw(ffi::sk_render_pass_descriptor_new_for_texture(
                texture.as_ptr(),
                clear_color.r.into(),
                clear_color.g.into(),
                clear_color.b.into(),
                clear_color.a.into(),
                load_action as i32,
                store_action as i32,
            ))
        }
    }
}

impl Renderer {
    /// Creates an `SKRenderer` backed by the given Metal device.
    #[must_use]
    pub fn new(device: &MetalDevice) -> Option<Self> {
        // SAFETY: sk_renderer_new is a Swift FFI function that returns a retained
        // SpriteKit object pointer (or null). The device pointer is valid because it comes
        // from apple-metal's MetalDevice. from_raw() handles the null case.
        unsafe { Self::from_raw(ffi::sk_renderer_new(device.as_ptr())) }
    }

    pub fn set_scene(&self, scene: Option<&Scene>) {
        // SAFETY: sk_renderer_set_scene is a Swift FFI function that accepts a valid
        // SKRenderer pointer (self.ptr) and an optional SKScene pointer. Both are guaranteed
        // to be valid because self and scene come from constructed handle types. null_mut()
        // is a valid representation for None.
        unsafe {
            ffi::sk_renderer_set_scene(
                self.ptr,
                scene.map_or(core::ptr::null_mut(), Scene::as_ptr),
            );
        }
    }

    /// Advances the scene simulation to `time`.
    pub fn update_at_time(&self, time: f64) {
        // SAFETY: sk_renderer_update_at_time accepts a valid SKRenderer pointer (self.ptr)
        // and a f64 time value. Both are safe to pass. The pointer is valid because self
        // is a constructed handle type. This is a pure simulation update with no output.
        unsafe { ffi::sk_renderer_update_at_time(self.ptr, time) };
    }

    /// Encodes a render pass into `command_buffer` using `pass_descriptor`.
    pub fn render(
        &self,
        viewport: CGRect,
        command_buffer: &CommandBuffer,
        pass_descriptor: &RenderPassDescriptor,
    ) {
        // SAFETY: sk_renderer_render accepts valid SKRenderer, MTLCommandBuffer, and
        // SKRenderPassDescriptor pointers, all guaranteed to be valid because they come
        // from constructed handle types. CGRect components are POD values safe to pass.
        unsafe {
            ffi::sk_renderer_render(
                self.ptr,
                viewport.x,
                viewport.y,
                viewport.width,
                viewport.height,
                command_buffer.as_ptr(),
                pass_descriptor.as_ptr(),
            );
        };
    }
}

/// Reads all pixel bytes from a Metal texture (BGRA8 assumed).
///
/// Returns `Ok(bytes)` where `bytes.len() == width * height * 4`.
pub fn read_texture_bytes(texture: &MetalTexture) -> Result<Vec<u8>, SpriteKitError> {
    let width = texture.width();
    let height = texture.height();
    let bytes_per_row = width
        .checked_mul(4)
        .ok_or_else(|| SpriteKitError::new("texture row byte count overflowed usize"))?;
    let byte_len = bytes_per_row
        .checked_mul(height)
        .ok_or_else(|| SpriteKitError::new("texture byte count overflowed usize"))?;
    let mut bytes = vec![0_u8; byte_len];
    // SAFETY: sk_texture_copy_bytes accepts a valid MTLTexture pointer (from apple-metal)
    // and a mutable buffer. The buffer is allocated with the correct size (checked above),
    // and as_mut_ptr() is safe on a vec we just created. The cast to *mut c_void is safe
    // because the FFI layer will interpret it as a buffer of u8s.
    let ok = unsafe {
        ffi::sk_texture_copy_bytes(texture.as_ptr(), bytes.as_mut_ptr().cast(), bytes_per_row)
    };
    if ok {
        Ok(bytes)
    } else {
        Err(SpriteKitError::new(
            "failed to copy texture bytes from MTLTexture",
        ))
    }
}
