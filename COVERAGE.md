# SpriteKit coverage map

This release tracks coverage by SpriteKit logical area rather than by every individual selector. The goal is to make it clear which areas have practical, headless-safe Rust wrappers today and which ones still have intentional gaps.

Status key:

- `✅ implemented` — the crate exposes a useful core surface for the logical area and it is covered by examples/tests.
- `🟡 partial` — the crate exposes a meaningful subset, but some constructors or API families are intentionally omitted.
- `⏭️ skipped` — intentionally not exposed in this release.

## Requested logical areas

| Area | Status | Rust modules | Example | Test | Notes |
| --- | --- | --- | --- | --- | --- |
| `SKScene` | ✅ implemented | `src/scene.rs`, `src/ffi/scene.rs` | `examples/02_scene_basics.rs` | `tests/scene_area.rs` | Size, scale mode, background color, anchor point, physics world, attached view, point conversion. |
| `SKNode` | ✅ implemented | `src/node.rs`, `src/ffi/node.rs` | `examples/03_node_hierarchy.rs` | `tests/node_area.rs` | Hierarchy, naming, frames, transforms, visibility, speed, physics body attachment, constraints, actions, point containment. |
| `SKAction` | ✅ implemented | `src/action.rs`, `src/ffi/action.rs` | `examples/04_action_basics.rs` | `tests/action_area.rs` | Core movement, rotation, scale, resize, fade, texture animation, grouping/repetition, timing properties, reverse. |
| `SKPhysicsBody` | ✅ implemented | `src/physics_body.rs`, `src/ffi/physics_body.rs` | `examples/05_physics_body_properties.rs` | `tests/physics_body_area.rs` | Shape constructors, compound bodies, material properties, masks, velocity, force/impulse/torque APIs. |
| `SKPhysicsWorld` | ✅ implemented | `src/physics_world.rs`, `src/ffi/physics_world.rs` | `examples/06_physics_world_queries.rs` | `tests/physics_world_area.rs` | Gravity, speed, body queries by point/rect/ray. |
| `SKPhysicsJoint` | 🟡 partial | `src/physics_joint.rs`, `src/ffi/physics_joint.rs` | `examples/07_physics_joint_basics.rs` | `tests/physics_joint_area.rs` | Covers pin/spring/fixed/sliding joints and shared reaction-force accessors. `SKPhysicsJointLimit` is intentionally skipped. |
| `SKLabelNode` | ✅ implemented | `src/label_node.rs`, `src/ffi/label_node.rs` | `examples/08_label_node_text.rs` | `tests/label_node_area.rs` | Text, font, size, alignment, line count, layout width, color/blend mode. |
| `SKSpriteNode` | ✅ implemented | `src/sprite_node.rs`, `src/ffi/sprite_node.rs` | `examples/09_sprite_node_features.rs` | `tests/sprite_node_area.rs` | Color/texture constructors, normal textures, size, anchor point, blend mode, light/shadow masks, center rect, shader attachment. |
| `SKView` | ✅ implemented | `src/view.rs`, `src/ffi/view.rs` | `examples/10_view_present_scene.rs` | `tests/view_area.rs` | Headless view creation, diagnostics flags, preferred FPS, transparency/sibling-order options, scene presentation, texture capture. |
| `SKConstraint` / `SKRange` | ✅ implemented | `src/constraint.rs`, `src/ffi/constraint.rs` | `examples/11_constraint_ranges.rs` | `tests/constraint_area.rs` | Range constructors, position/distance/orientation/zRotation constraints, enabled flag, reference node. |
| `SKKeyframeSequence` | 🟡 partial | `src/keyframe.rs`, `src/ffi/keyframe.rs` | `examples/12_keyframe_sequence.rs` | `tests/keyframe_area.rs` | Scalar keyframes, interpolation mode, repeat mode, sampling, mutation. Non-scalar payload types are not wrapped yet. |
| `SKEmitterNode` | 🟡 partial | `src/emitter_node.rs`, `src/ffi/emitter_node.rs` | `examples/13_emitter_node_particles.rs` | `tests/emitter_node_area.rs` | Core particle texture, blend, position, speed, birth/lifetime, size/scale/alpha, target node, field bitmask. Advanced action/keyframe-based particle properties are not wrapped yet. |
| `SKShader` | 🟡 partial | `src/shader.rs`, `src/ffi/shader.rs` | `examples/14_shader_uniforms.rs` | `tests/shader_area.rs` | Source strings, float uniforms, attribute counting. Vector/matrix/color uniform helpers are not wrapped yet. |
| `SKAudioNode` | 🟡 partial | `src/audio_node.rs`, `src/ffi/audio_node.rs` | `examples/15_audio_node_basic.rs` | `tests/audio_node_area.rs` | Default AVAudioNode-backed constructor, autoplay loop, positional audio, AVAudioNode presence. File/URL constructors are not wrapped yet. |
| `SKVideoNode` | 🟡 partial | `src/video_node.rs`, `src/ffi/video_node.rs` | `examples/16_video_node_basic.rs` | `tests/video_node_area.rs` | Default AVPlayer-backed constructor, play/pause, size, anchor point. File/URL constructors are not wrapped yet. |
| `SKLightNode` | 🟡 partial | `src/light_node.rs`, `src/ffi/light_node.rs` | `examples/17_light_node_basic.rs` | `tests/light_node_area.rs` | Enabled, light/ambient/shadow color setters, falloff, category bitmask. The surface is intentionally focused on common lighting controls. |
| `SK3DNode` | 🟡 partial | `src/three_d_node.rs`, `src/ffi/three_d_node.rs` | `examples/18_three_d_node_basic.rs` | `tests/three_d_node_area.rs` | Viewport size, empty SceneKit scene creation, scene time, play/loop flags, default lighting. Arbitrary SceneKit scene bridging is not exposed yet. |

## Existing supplemental areas

| Area | Status | Rust modules | Example | Notes |
| --- | --- | --- | --- | --- |
| `SKEffectNode` | ✅ implemented | `src/effect_node.rs`, `src/ffi/effect_node.rs` | `examples/01_offline_render_smoke.rs` | Existing v0.1.0 coverage retained. |
| `SKTexture` | ✅ implemented | `src/texture.rs`, `src/ffi/texture.rs` | `examples/09_sprite_node_features.rs` | Existing texture surface retained and reused by sprite/emitter/shader flows. |
| `SKRenderer` | ✅ implemented | `src/renderer.rs`, `src/ffi/renderer.rs` | `examples/01_offline_render_smoke.rs` | Offline rendering into `apple-metal` textures remains supported. |

## Intentionally skipped APIs

| API | Status | Reason |
| --- | --- | --- |
| `SKPhysicsJointLimit` | ⏭️ skipped | Direct headless validation triggered a PhysicsKit crash, so it is left out of the safe wrapper surface for `0.2.0`. |
| `SKPhysicsWorld.addJoint` / `removeJoint` | ⏭️ skipped | Headless joint insertion/removal crashed during validation; examples/tests only exercise joint creation and property access. |

## Validation

The coverage above is backed by:

- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Running every example in `examples/*.rs`
