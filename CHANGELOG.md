# Changelog

All notable changes to `spritekit-rs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- `PhysicsWorld` keeps its contact delegates alive. `SKPhysicsWorld.contactDelegate` is declared `assign`, so dropping a `PhysicsContactDelegate` while the scene simulated left SpriteKit calling a freed bridge object and closure, and `has_contact_delegate` read the dangling pointer. SpriteKit also reads the property on the simulation thread, so releasing a delegate as soon as another thread cleared or replaced it could free it under a callback. Dropping the Rust handle now only stops the callbacks, and the world keeps every contact delegate set on it until the world is freed. `tests/delegate_race_tests.rs` swaps delegates on one thread while another simulates.
- `Renderer::render` encoded into whatever command buffer it was given; Metal aborts the process when `SKRenderer` encodes into a committed command buffer or one with an active encoder. See Changed.
- `read_texture_bytes` sizes its buffer with `apple_metal::bytes_per_pixel`, passes the buffer length to the bridge and checks the copy. It used to assume 4 bytes per pixel, so RGBA16Float, RGBA32Float and BGRA10_XR textures overflowed the heap. Compressed, depth/stencil, framebuffer-only, non-2D, and private or memoryless textures are rejected.

### Fixed

- `NodeExt::add_child` rejects children that already have a parent, self-parenting and cycles, and `move_to_parent` rejects moves under the node itself or a descendant, instead of letting SpriteKit raise `NSInvalidArgumentException` and abort the process.
- Contact callbacks can read the contact. SpriteKit delivers a private contact class that failed the bridge's `SKPhysicsContact` cast, so `body_a`, `body_b`, `contact_point`, `contact_normal` and `collision_impulse` always returned `None` or zero.
- `Renderer::new` serializes `SKRenderer` creation. SpriteKit's shader loading is not thread-safe, and creating renderers on several threads at once crashed.
- `MutableTexture::modify_pixel_data` and the `TextureAtlas` completion-handler preloads release their closures when SpriteKit drops the block without calling it, instead of leaking them.
- The `async` atlas preload futures stayed pending forever when SpriteKit released the completion handler without calling it; they now resolve with an error.
- Swift bridge conversions of enum raw values, particle render order and tile placement weights no longer trap on out-of-range values.

### Changed

- **Breaking:** `Renderer::render(viewport, &CommandQueue, &RenderPassDescriptor)` encodes the frame into a new command buffer and returns it uncommitted as `Result<CommandBuffer, SpriteKitError>`, so nothing else can use the buffer while SpriteKit encodes. The `unsafe` `Renderer::render_into` encodes into an existing command buffer. Both return errors for committed or failed command buffers and for mismatched Metal devices instead of aborting. The raw `ffi::sk_renderer_render` takes an error out-pointer and returns `bool`.
- **Breaking:** `TextureAtlas::preload_async` and `preload_texture_atlases_async` return `impl Future<Output = Result<(), SpriteKitError>>`.
- **Breaking:** clearing or replacing a contact delegate no longer frees its closures; they are freed with the physics world. Dropping the Rust handle still stops them at once.
- **Breaking:** `NodeExt::add_child` and `NodeExt::move_to_parent` return `Result<(), SpriteKitError>`.
- **Breaking:** `View::with_frame` returns `Result<View, SpriteKitError>` and fails off the main thread. Other `SKView` calls do nothing and return defaults off the main thread, and `Scene::view` returns `None` there.
- **Breaking:** raw `ffi` declarations follow the bridge: `sk_node_add_child` and `sk_node_move_to_parent` return `bool` with an error out-pointer; the delegate constructors, `sk_mutable_texture_modify_pixel_data` and the atlas preload functions take a context-release callback; `sk_scene_delegate_new` takes the four new phase callbacks; `sk_texture_copy_bytes` takes the buffer length and bytes per pixel; callback parameters are `unsafe extern "C" fn` pointers.
- `PhysicsContactDelegate`, `SceneDelegate` and `ViewDelegate` keep their closures in a `doom_fish_utils::callback_context::CallbackContext`; dropping a handle deactivates its callbacks immediately.
- **Breaking:** require `apple-cf` 0.11 and `apple-metal` 0.10, whose types appear in this crate's API (`CGRect`, `MetalDevice`, `MetalTexture`, `CommandBuffer`), and `doom-fish-utils` 0.4.1. `rust-version` rises from 1.76 to 1.82.

### Added

- `Action::run_block`, and `Action::custom`, whose block receives `&Node` and the elapsed time.
- `NodeExt::run_action_with_completion`, `run_action_with_key`, `action_for_key` and `remove_action_for_key`.
- `SceneDelegateCallbacks` and `SceneDelegate::from_callbacks`, adding `didEvaluateActions`, `didSimulatePhysics`, `didApplyConstraints` and `didFinishUpdate` to `update`.
- `PhysicsWorld::add_joint`, `remove_joint` and `remove_all_joints`. `add_joint` rejects joints that were already added and joints whose bodies are not attached to nodes in a scene, which SpriteKit crashes on.
- `color::verify_ffi_layout` and compile-time size and alignment checks for `Color`, which crosses the bridge by value.
- `Renderer::render_into`.
- Tests that drop delegates while they are set and then drive the simulation, check frame-phase order, action callbacks, hierarchy and joint errors, texture readback (RGBA16Float, BGRA10_XR and rejected formats) and closure release. `SKView` tests run on the main thread in `tests/main_thread.rs`. Further tests swap contact delegates while another thread simulates, check command-buffer states for rendering, and drop atlas preload completions through the bridge.

### Removed

- The empty `SpriteKitBridge.h` header, its module map and the `publicHeadersPath` setting of the Swift package.

## [0.3.3] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.2] - 2026-05-20

### Added

- Wrapped `SKMutableTexture.modifyPixelDataWithBlock:` as `MutableTexture::modify_pixel_data`.
- Wrapped `SKTextureAtlas` completion-handler preloading APIs, including the named-atlas result callback.
- Added an optional `async` feature with `TextureAtlas::*_async` helpers for atlas preloading callbacks.

### Notes

- Phase 32 completeness + async sweep.
- Bumped the crate version from `0.3.1` to `0.3.2`, refreshed the coverage docs, and added headless smoke coverage for the new callback and async surfaces.

## [0.3.1] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.3.0] - 2026-05-18

### Changed

- Updated `apple-cf` to the `0.8.x` release line and widened the Metal dependency range so Cargo can resolve the new dependency graph
- Migrated `CGRect` field access to the nested `origin`/`size` layout and replaced struct literals with `CGRect::new(...)`
- Bumped the crate version for the `apple-cf 0.8` compatibility release

## [0.2.3] - 2026-05-18

### Changed

- Added `///` coverage across the public SpriteKit surface, raising measured public-item documentation coverage from 0.8% to 100.0%
- Bumped the patch version for the documentation pass release

## [0.2.2] - 2026-05-18

### Changed

- Added `// SAFETY:` comments to all FFI unsafe blocks for improved audibility and correctness documentation

## [0.2.1] - 2026-05-17

### Added

- Closed the remaining macOS SpriteKit symbol gaps with new wrappers for attributes, event helpers, regions, reach constraints, transitions, camera/crop/field/reference/shape/transform nodes, mutable textures, texture atlases, physics contacts/delegates, tile APIs, warp geometry, and view/scene delegates
- Added `SKAction` helpers for physics-body, playback, mixer, audio-node, and warpable categories
- Added `tests/coverage_fill_area.rs` smoke coverage for the newly exposed surface, bringing the audit to full symbol coverage across the public macOS SpriteKit headers

### Changed

- Promoted `SKPhysicsJointLimit` from an audit gap to a wrapped type while keeping headless crash-prone validation paths out of the default test flow
- Updated README, coverage docs, and `COVERAGE_AUDIT.md` for the `0.2.1` release and 100% symbol-level coverage

## [0.2.0] - 2026-05-16

### Added

- Split the Rust FFI into per-area modules and matched it with per-area Swift bridge files using the retained-handle `@_cdecl` pattern from `screencapturekit-rs`
- Expanded wrapper coverage across the requested SpriteKit logical areas: `SKScene`, `SKNode`, `SKAction`, `SKPhysicsBody`, `SKPhysicsWorld`, `SKPhysicsJoint`, `SKLabelNode`, `SKSpriteNode`, `SKView`, `SKConstraint`, `SKKeyframeSequence`, `SKEmitterNode`, `SKShader`, `SKAudioNode`, `SKVideoNode`, `SKLightNode`, and `SK3DNode`
- Added focused examples `examples/02_scene_basics.rs` through `examples/18_three_d_node_basic.rs`
- Added focused integration tests `tests/*_area.rs` with at least one test per requested logical area
- Added `COVERAGE.md` documenting implemented, partial, and intentionally skipped areas

### Changed

- Linked the bridge against `AVFoundation` and `SceneKit` for the new media and `SK3DNode` wrappers
- Updated crate metadata, README status, and release documentation for `0.2.0`

## [0.1.0] - 2025-01-01

### Added

- `Scene` — `SKScene` wrapper: `with_size`, `size`, `setSize`, `scaleMode`, `backgroundColor`, `anchorPoint`, `physicsWorld`
- `Node` — `SKNode` wrapper: position, zPosition, zRotation, xScale/yScale/setScale, alpha, hidden, paused, speed, add/remove child, physics body, run/remove actions
- `SpriteNode` — `SKSpriteNode`: create from texture or color+size, texture, size, anchorPoint, color, colorBlendFactor, blendMode
- `Texture` — `SKTexture`: `imageNamed`, `fromCGImage`, `fromRgbaBytes`, subrect, size, filteringMode, usesMipmaps
- `Action` — `SKAction`: moveBy/To, rotateBy/To, scaleBy/To, resizeTo, fadeIn/Out/To/By, hide/unhide, setTexture, animateWithTextures, wait, sequence, group, repeat, repeatForever
- `PhysicsBody` — `SKPhysicsBody`: circle, rect, edgeLoopRect, fromTexture; full property surface including bitmasks, velocity, applyForce, applyImpulse
- `PhysicsWorld` — gravity and speed
- `EffectNode` — `SKEffectNode`: shouldEnableEffects, shouldRasterize, blendMode
- `Renderer` + `RenderPassDescriptor` — offline `SKRenderer` rendering into Metal textures
- `read_texture_bytes` helper — read pixels back from a Metal texture
- `NodeExt` trait — blanket node API shared across `Scene`, `Node`, `SpriteNode`, `EffectNode`
- `Color` helper with named constructors
- Smoke example `examples/01_offline_render_smoke.rs`
