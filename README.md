# spritekit-rs

Safe Rust bindings for Apple's [SpriteKit](https://developer.apple.com/documentation/spritekit) framework on macOS.

> **Status:** v0.2.0 expands the bridge to 17 SpriteKit logical areas using the `screencapturekit-rs` handle-based pattern. The crate now covers scenes, nodes, actions, physics bodies/world/joints, label/sprite/view/constraint/keyframe/emitter/shader/audio/video/light/3D nodes, effect nodes, and offline rendering through `SKRenderer` into Metal textures.

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

- Bridge architecture follows the `screencapturekit-rs` pattern: per-area Rust modules, per-area Swift bridge files, `@_cdecl` exports, retained opaque handles, and explicit `sk_release` cleanup.
- Scene graph coverage includes `SKScene`, `SKNode`, `SKSpriteNode`, `SKLabelNode`, `SKEffectNode`, `SKLightNode`, and `SK3DNode`, plus shared `NodeExt` helpers for hierarchy, transforms, actions, physics bodies, and constraints.
- Animation and simulation coverage includes `SKAction`, `SKPhysicsBody`, `SKPhysicsWorld`, headless-safe `SKPhysicsJoint` wrappers (`pin`, `spring`, `fixed`, `sliding`), `SKConstraint`/`SKRange`, and `SKKeyframeSequence`.
- Rendering and assets coverage includes `SKTexture`, `SKShader`, `SKEmitterNode`, `SKView`, and offline `SKRenderer` rendering into `apple-metal` textures.
- Media coverage includes `SKAudioNode` and `SKVideoNode` with asset-free AVFoundation-backed constructors for examples/tests.
- The repository now includes 18 numbered examples under `examples/` and 17 focused integration tests under `tests/`, with at least one example and one test per requested logical area.
- Detailed area-by-area status, including intentionally skipped APIs, lives in [`COVERAGE.md`](COVERAGE.md).

## Examples, tests, and coverage

```bash
cargo test
cargo run --example 01_offline_render_smoke
cargo run --example 10_view_present_scene
cargo run --example 18_three_d_node_basic
```

- `examples/01_*.rs` through `examples/18_*.rs` provide headless smoke coverage for the renderer plus every requested `SpriteKit` logical area.
- `tests/*_area.rs` exercise the same surfaces under `cargo test`.
- [`COVERAGE.md`](COVERAGE.md) records which logical areas are fully wrapped, partially wrapped, or intentionally skipped.

## Coverage notes

- `SKPhysicsJointLimit` and headless `SKPhysicsWorld.addJoint/removeJoint` flows are intentionally omitted because `PhysicsKit` crashed during direct validation; `pin`, `spring`, `fixed`, and `sliding` joints are covered.
- `SKAudioNode` / `SKVideoNode` currently use default AVFoundation-backed constructors so examples and tests stay asset-free and deterministic.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
