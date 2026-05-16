use apple_cf::cg::{CGPoint, CGRect, CGSize};
use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use spritekit::{
    read_texture_bytes, Color, NodeExt, RenderPassDescriptor, Renderer, Scene, SceneScaleMode,
    SpriteNode,
};

const WIDTH: usize = 128;
const HEIGHT: usize = 128;
const WIDTH_F: f64 = 128.0;
const HEIGHT_F: f64 = 128.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = MetalDevice::system_default().ok_or("no Metal device available")?;
    let queue = device
        .new_command_queue()
        .ok_or("failed to create command queue")?;

    // Create a shared BGRA8 render-target texture.
    let texture = device
        .new_texture(TextureDescriptor {
            pixel_format: pixel_format::BGRA8UNORM,
            width: WIDTH,
            height: HEIGHT,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
        })
        .ok_or("failed to allocate render target texture")?;

    // Build a 128×128 scene with a black background.
    let scene =
        Scene::with_size(CGSize::new(WIDTH_F, HEIGHT_F)).ok_or("failed to create SKScene")?;
    scene.set_background_color(Color::black());
    scene.set_scale_mode(SceneScaleMode::AspectFit);
    // Centre the coordinate origin.
    scene.set_anchor_point(CGPoint::new(0.5, 0.5));

    // Add a red 32×32 sprite at the centre.
    let sprite = SpriteNode::with_color(Color::red(), CGSize::new(32.0, 32.0))
        .ok_or("failed to create sprite node")?;
    sprite.set_position(CGPoint::new(0.0, 0.0));
    scene.add_child(&sprite);

    // Create SKRenderer and wire up the scene.
    let renderer = Renderer::new(&device).ok_or("failed to create SKRenderer")?;
    renderer.set_scene(Some(&scene));

    // Build render-pass and command buffer.
    let pass = RenderPassDescriptor::for_texture(&texture, Color::black())
        .ok_or("failed to create render pass descriptor")?;
    let command_buffer = queue
        .new_command_buffer()
        .ok_or("failed to create command buffer")?;

    // Step the simulation then render.
    renderer.update_at_time(0.0);
    renderer.render(
        CGRect::new(0.0, 0.0, WIDTH_F, HEIGHT_F),
        &command_buffer,
        &pass,
    );

    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Read pixels back and assert at least one non-zero byte.
    let pixels = read_texture_bytes(&texture)?;
    assert!(
        pixels.iter().any(|&byte| byte != 0),
        "rendered texture was entirely zeroes — render may have failed"
    );

    println!(
        "✅  spritekit-rs offline render OK  ({WIDTH}×{HEIGHT} BGRA8, {} bytes)",
        pixels.len()
    );
    Ok(())
}
