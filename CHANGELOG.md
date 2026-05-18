# Changelog

All notable changes to `spritekit-rs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
