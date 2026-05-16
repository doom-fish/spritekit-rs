# Changelog

All notable changes to `spritekit-rs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
