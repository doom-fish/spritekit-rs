use apple_cf::cg::CGRect;
use apple_metal::{
    command_buffer_status, storage_mode, texture_type, CommandBuffer, CommandBufferError,
    CommandQueue, MetalDevice, MetalTexture,
};

use crate::color::Color;
use crate::error::{bridge_result, SpriteKitError};
use crate::ffi;
use crate::private::handle_type;
use crate::scene::Scene;

handle_type!(RenderPassDescriptor);
handle_type!(Renderer);

/// Enum for `SKRenderer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

/// Enum for `SKRenderer`.
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

    /// Wraps `SKRenderPassDescriptor`.
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

    /// Sets a property exposed by `SKRenderer`.
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

    /// Encodes a render pass into a new command buffer from `queue` using `pass_descriptor`.
    pub fn render(
        &self,
        viewport: CGRect,
        queue: &CommandQueue,
        pass_descriptor: &RenderPassDescriptor,
    ) -> Result<CommandBuffer, SpriteKitError> {
        let command_buffer = queue.new_command_buffer().ok_or_else(|| {
            SpriteKitError::new("MTLCommandQueue could not create a command buffer")
        })?;
        self.render_into(viewport, &command_buffer, pass_descriptor)?;
        Ok(command_buffer)
    }

    pub fn render_into(
        &self,
        viewport: CGRect,
        command_buffer: &CommandBuffer,
        pass_descriptor: &RenderPassDescriptor,
    ) -> Result<(), SpriteKitError> {
        let mut error = core::ptr::null_mut();
        let rendered = command_buffer
            .encode_foreign(|foreign| unsafe {
                ffi::sk_renderer_render(
                    self.ptr,
                    viewport.origin.x,
                    viewport.origin.y,
                    viewport.size.width,
                    viewport.size.height,
                    foreign.command_buffer(),
                    pass_descriptor.as_ptr(),
                    &raw mut error,
                )
            })
            .map_err(|refused| refused_command_buffer(command_buffer, &refused))?;
        unsafe { bridge_result(rendered, error, "SKRenderer refused to render") }
    }
}

fn refused_command_buffer(
    command_buffer: &CommandBuffer,
    refused: &CommandBufferError,
) -> SpriteKitError {
    match (refused, command_buffer.status()) {
        (CommandBufferError::InvalidState { .. }, command_buffer_status::ERROR) => {
            SpriteKitError::new(format!(
                "the command buffer failed: {}",
                command_buffer
                    .error()
                    .unwrap_or_else(|| "Metal reported no error message".to_owned())
            ))
        }
        (CommandBufferError::InvalidState { .. }, status) => SpriteKitError::new(format!(
            "the command buffer was already committed (status {status})"
        )),
        (CommandBufferError::ActiveEncoder, _) => {
            SpriteKitError::new("another command encoder is still open on the command buffer")
        }
        (refused, _) => {
            SpriteKitError::new(format!("the command buffer refused the render: {refused}"))
        }
    }
}

/// Reads mipmap level 0 of a 2D texture, sized by its pixel format.
///
/// Compressed, depth, stencil and framebuffer-only textures, and textures without
/// shared or managed storage, are rejected.
pub fn read_texture_bytes(texture: &MetalTexture) -> Result<Vec<u8>, SpriteKitError> {
    use apple_metal::pixel_format::{
        DEPTH16UNORM, DEPTH32FLOAT, STENCIL8, X24_STENCIL8, X32_STENCIL8,
    };

    let format = texture.pixel_format();
    if matches!(
        format,
        DEPTH16UNORM | DEPTH32FLOAT | STENCIL8 | X24_STENCIL8 | X32_STENCIL8
    ) {
        return Err(SpriteKitError::new(format!(
            "pixel format {format} is a depth or stencil format"
        )));
    }
    let bytes_per_pixel = apple_metal::bytes_per_pixel(format).ok_or_else(|| {
        SpriteKitError::new(format!(
            "pixel format {format} is compressed or has no per-pixel layout"
        ))
    })?;
    let mode = texture.storage_mode();
    if !matches!(mode, storage_mode::SHARED | storage_mode::MANAGED) {
        return Err(SpriteKitError::new(format!(
            "texture storage mode {mode} is not readable by the CPU"
        )));
    }
    if texture.texture_type() != texture_type::TYPE_2D {
        return Err(SpriteKitError::new(format!(
            "texture type {} is not a 2D texture",
            texture.texture_type()
        )));
    }
    let width = texture.width();
    let height = texture.height();
    let bytes_per_row = width
        .checked_mul(bytes_per_pixel)
        .ok_or_else(|| SpriteKitError::new("texture row byte count overflowed usize"))?;
    let byte_len = bytes_per_row
        .checked_mul(height)
        .ok_or_else(|| SpriteKitError::new("texture byte count overflowed usize"))?;
    let mut bytes = vec![0_u8; byte_len];
    let copied = unsafe {
        ffi::sk_texture_copy_bytes(
            texture.as_ptr(),
            bytes.as_mut_ptr().cast(),
            bytes.len(),
            bytes_per_row,
            bytes_per_pixel,
        )
    };
    if copied {
        Ok(bytes)
    } else {
        Err(SpriteKitError::new(
            "MTLTexture refused the CPU copy (framebuffer-only, wrong layout or storage)",
        ))
    }
}
