# spritekit-rs

Safe Rust bindings for Apple's [SpriteKit](https://developer.apple.com/documentation/spritekit) framework on macOS.

> **Status:** v0.4.0 retains the physics contact delegate while it is set, sizes texture readback by pixel format, returns errors instead of `SpriteKit` exceptions for invalid hierarchy and joint operations, and keeps `SKView` on the main thread. It adds action completion, block, custom and keyed actions, the full scene frame-phase delegate and physics-joint management. Every public macOS `SpriteKit` type, protocol and category is reachable from Rust, but this is symbol-level coverage, not selector-for-selector parity.

## Installation

```toml
[dependencies]
spritekit-rs = "0.4"
```

The library is imported as `spritekit`. It needs macOS 11 or later, Rust 1.82 or later, and a Swift toolchain (Xcode or the Command Line Tools), which `build.rs` uses to compile the bundled Swift bridge.

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
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
            ..TextureDescriptor::new_2d(256, 256, pixel_format::BGRA8UNORM)
        })
        .expect("texture");

    let scene = Scene::with_size(CGSize::new(256.0, 256.0)).expect("scene");
    scene.set_background_color(Color::black());
    scene.set_scale_mode(SceneScaleMode::AspectFit);
    scene.set_anchor_point(CGPoint::new(0.5, 0.5));

    let sprite = SpriteNode::with_color(Color::green(), CGSize::new(64.0, 64.0)).expect("sprite");
    sprite.set_position(CGPoint::new(0.0, 0.0));
    scene.add_child(&sprite)?;

    let renderer = Renderer::new(&device).expect("renderer");
    renderer.set_scene(Some(&scene));

    let pass = RenderPassDescriptor::for_texture(&texture, Color::black()).expect("pass");
    renderer.update_at_time(0.0);
    let command_buffer = renderer.render(CGRect::new(0.0, 0.0, 256.0, 256.0), &queue, &pass)?;
    command_buffer.commit()?;
    command_buffer.wait_until_completed()?;

    let pixels = spritekit::read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&b| b != 0));
    Ok(())
}
```

## Highlights

- Bridge architecture follows the `screencapturekit-rs` pattern: per-area Rust modules, per-area Swift bridge files, `@_cdecl` exports, retained opaque handles, and explicit `sk_release` cleanup.
- Scene graph coverage now spans `SKScene`, `SKNode`, `SKCameraNode`, `SKCropNode`, `SKShapeNode`, `SKTransformNode`, `SKSpriteNode`, `SKLabelNode`, `SKEffectNode`, `SKLightNode`, `SKFieldNode`, `SKReferenceNode`, and `SK3DNode`, plus shared `NodeExt` helpers for hierarchy, transforms, accessibility, actions, physics bodies, and reach constraints.
- Animation and simulation coverage includes `SKAction` plus the physics-body, playback, mixer, audio-node, and warpable categories, block and custom actions, completion handlers and keyed actions, `SKPhysicsBody`, `SKPhysicsWorld` with joint management, `SKPhysicsContact`/delegate bridging, `SKPhysicsJoint` wrappers including `SKPhysicsJointLimit`, `SKConstraint`/`SKRange`, and `SKKeyframeSequence`.
- Rendering and asset coverage includes `SKTexture`, `SKMutableTexture` pixel-data updates, `SKTextureAtlas` lookup plus completion-handler and optional `async` preloading, `SKShader`, `SKAttribute`, `SKAttributeValue`, `SKTransition`, tile-map/tile-set APIs, warp geometry, `SKView`, and offline `SKRenderer` rendering into `apple-metal` textures.
- Media coverage includes `SKAudioNode` and `SKVideoNode` with asset-free AVFoundation-backed constructors for examples/tests.
- The repository includes 18 numbered examples under `examples/` and 24 integration test targets under `tests/`, including delegate-lifetime, action-callback, hierarchy/joint and texture-readback tests driven through `SKRenderer`.
- Detailed logical-area status lives in [`COVERAGE.md`](COVERAGE.md), and the full header audit lives in [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md).

## Threads, lifetimes and validation

- `SKView` is main-thread only. `View::with_frame` returns an error on other threads, view calls made there do nothing and return defaults, and `Scene::view` returns `None` off the main thread. `View` is not `Send`.
- `SKPhysicsWorld.contactDelegate` is an unretained (`assign`) property, so `PhysicsWorld` retains the delegate while it is set. Dropping a `PhysicsContactDelegate` stops its callbacks at once; the bridge object is released when the delegate is cleared or replaced, or when the world is freed. `SpriteKit` frees a scene's physics world on the main queue when the scene is released on another thread.
- Scene and view delegates are weak references in `SpriteKit`: keep the Rust `SceneDelegate` or `ViewDelegate` alive for as long as you want callbacks.
- Delegate and action closures must be `Send + Sync`, and completion closures `Send`, because `SpriteKit` may call them on its own threads. Panics are caught at the FFI boundary. Closures that `SpriteKit` drops without calling are still released.
- `NodeExt::add_child` and `move_to_parent` return an error for children that already have a parent, for self-parenting and for cycles, instead of letting `SpriteKit` raise an exception. `PhysicsWorld::add_joint` rejects joints that were already added and joints whose bodies are not attached to nodes in a scene, which `SpriteKit` would crash on.
- `read_texture_bytes` sizes its buffer with `apple_metal::bytes_per_pixel` and rejects compressed, depth/stencil, framebuffer-only, non-2D and private or memoryless textures.
- `Renderer::new` serializes `SKRenderer` creation, whose shader loading is not thread-safe. Drive `Renderer::update_at_time` with a positive, increasing clock: `SpriteKit` does not advance actions or physics from a zero time base.

## Examples, tests, and coverage

```bash
cargo test --all-features
cargo run --example 01_offline_render_smoke
cargo run --example 10_view_present_scene
cargo run --example 18_three_d_node_basic
```

- `examples/01_*.rs` through `examples/18_*.rs` provide headless smoke coverage for the renderer plus every requested `SpriteKit` logical area.
- `tests/*_area.rs`, including `tests/coverage_fill_area.rs`, exercise the same surfaces under `cargo test`. `tests/main_thread.rs` runs the `SKView` tests on the main thread (`harness = false`).
- [`COVERAGE.md`](COVERAGE.md) records logical-area status, while [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) records the symbol-level header audit: a symbol counts as verified when at least one Rust API reaches it.

## Coverage notes

- `SKMutableTexture` exposes `modify_pixel_data`, and `SKTextureAtlas` exposes completion-handler preloading plus an optional `async` feature for atlas preload futures.
- `SKKeyframeSequence` wraps scalar payloads only, `SKShader` wraps float uniforms only, and `SKAudioNode` / `SKVideoNode` use default AVFoundation-backed constructors so examples and tests stay asset-free; file and URL constructors are not wrapped.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
