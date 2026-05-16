# spritekit-rs

Safe Rust bindings for Apple's [SpriteKit](https://developer.apple.com/documentation/spritekit) framework on macOS.

> **Status:** v0.1.0 covers 2D scene construction, node hierarchy management, sprite nodes, textures, actions, physics bodies and world, effect nodes, and offline rendering through `SKRenderer` into Metal textures.

## Quick start

```rust,no_run
use apple_cf::cg::{CGPoint, CGRect, CGSize};
use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use spritekit::{
    read_texture_bytes, Color, NodeExt, RenderPassDescriptor, Renderer, Scene,
    SceneScaleMode, SpriteNode,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = MetalDevice::system_default().expect("no Metal device");
    let queue = device.new_command_queue().expect("no command queue");
    let texture = device
        .new_texture(TextureDescriptor {
            pixel_format: pixel_format::BGRA8UNORM,
            width: 256,
            height: 256,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
        })
        .expect("texture");

    let scene = Scene::with_size(CGSize::new(256.0, 256.0)).expect("scene");
    scene.set_background_color(Color::black());
    scene.set_scale_mode(SceneScaleMode::AspectFit);
    scene.set_anchor_point(CGPoint::new(0.5, 0.5));

    let sprite = SpriteNode::with_color(Color::green(), CGSize::new(64.0, 64.0)).expect("sprite");
    sprite.set_position(CGPoint::new(0.0, 0.0));
    scene.add_child(&sprite);

    let renderer = Renderer::new(&device).expect("renderer");
    renderer.set_scene(Some(&scene));

    let pass = RenderPassDescriptor::for_texture(&texture, Color::black()).expect("pass");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    renderer.update_at_time(0.0);
    renderer.render(
        CGRect::new(0.0, 0.0, 256.0, 256.0),
        &command_buffer,
        &pass,
    );
    command_buffer.commit();
    command_buffer.wait_until_completed();

    let pixels = spritekit::read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&b| b != 0));
    Ok(())
}
```

## Highlights

- `Scene::with_size` — create a `SKScene` of a given point size; set background colour, scale mode, and anchor point; access the `PhysicsWorld`
- `Node` — base 2-D node with `position`, `zPosition`, `zRotation`, `xScale`/`yScale`/`setScale`, `alpha`, `hidden`, `paused`, `speed`, hierarchy and action APIs
- `SpriteNode` — create from a `Texture` or a solid colour+size; get/set `texture`, `size`, `anchorPoint`, `color`, `colorBlendFactor`, `blendMode`
- `Texture` — load by name, from a `CGImage` pointer, or from raw RGBA bytes; subrect cropping; `filteringMode`, `usesMipmaps`
- `Action` — `moveBy/To`, `rotateBy/To`, `scaleBy/To`, `resizeTo`, `fadeIn/Out/To/By`, `hide/unhide`, `setTexture`, `animateWithTextures`, `wait`, `sequence`, `group`, `repeat`, `repeatForever`
- `PhysicsBody` — `circle`, `rect`, `edgeLoopRect`, `fromTexture`; `isDynamic`, `allowsRotation`, `preciseCollision`, `pinned`, `friction`, `restitution`, `linearDamping`, `angularDamping`, `density`, `mass`, `affectedByGravity`, category/contact/collision bitmasks, `velocity`, `applyForce`, `applyImpulse`
- `PhysicsWorld` — `gravity`, `speed`
- `EffectNode` — `shouldEnableEffects`, `shouldRasterize`, `blendMode`
- `Renderer` + `RenderPassDescriptor` — offline `SKRenderer` rendering into `apple-metal` textures via `updateAtTime` + `render`
- `NodeExt` trait — all node types (`Scene`, `Node`, `SpriteNode`, `EffectNode`) share `addChild`, `removeFromParent`, position/scale/alpha methods, physics and action APIs via a blanket impl
- `apple_cf::cg::{CGPoint, CGSize, CGRect, CGVector}` geometry types — no duplicated 2-D structs

## Smoke example

```bash
cargo run --example 01_offline_render_smoke
```

Creates a 128×128 `SpriteKit` scene with a red sprite at the centre, renders it into a Metal texture via `SKRenderer`, reads the pixels back, and asserts at least one non-zero byte.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
