# spritekit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 75
VERIFIED: 75
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

Re-audited all 75 symbols enumerated in the v1 audit against SpriteKit.framework headers in MacOSX26.2.sdk. Verified via header grep for @interface, @protocol, @property, and category declarations. Re-confirmed all EXEMPT entries: UITouch (SKNodeTouches) and SKNodeFocusBehavior enum remain API_UNAVAILABLE(macos) in 26.2 and are correctly excluded. All 75 VERIFIED symbols found in crate's swift-bridge FFI (32 Swift modules) and safe Rust wrapper APIs. No new symbol-level coverage gaps detected.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `NSEvent (SKNodeEvent)` | category | `SKNode.h` | `Event::{mouse_moved, location_in_node}` |
| `SK3DNode` | interface | `SK3DNode.h` | `ThreeDNode` |
| `SKAction` | interface | `SKAction.h` | `Action` |
| `SKAction (MixerControl)` | category | `SKAction.h` | `Action::change_volume_to` |
| `SKAction (NodeWithPhysicsBody)` | category | `SKAction.h` | `Action::{change_charge_to, apply_force}` |
| `SKAction (PlaybackControl)` | category | `SKAction.h` | `Action::play` |
| `SKAction (SKActions)` | category | `SKAction.h` | `Action::{move_*, rotate_*, scale_*, resize_*, fade_*, set_texture, animate_with_textures, wait, sequence, group, repeat_*}` |
| `SKAction (SKAudioNode)` | category | `SKAudioNode.h` | `Action::stereo_pan_to` |
| `SKAction (SKWarpable)` | category | `SKWarpGeometry.h` | `Action::{warp_to, animate_with_warps}` |
| `SKActionTimingMode` | enum | `SKAction.h` | `ActionTimingMode` |
| `SKAttribute` | interface | `SKAttribute.h` | `Attribute` |
| `SKAttributeType` | enum | `SKAttribute.h` | `AttributeType` |
| `SKAttributeValue` | interface | `SKAttribute.h` | `AttributeValue` |
| `SKAudioNode` | interface | `SKAudioNode.h` | `AudioNode` |
| `SKBlendMode` | enum | `SKNode.h` | `BlendMode` |
| `SKCameraNode` | interface | `SKCameraNode.h` | `CameraNode + Scene::{camera, set_camera}` |
| `SKConstraint` | interface | `SKConstraint.h` | `Constraint` |
| `SKCropNode` | interface | `SKCropNode.h` | `CropNode` |
| `SKEffectNode` | interface | `SKEffectNode.h` | `EffectNode` |
| `SKEmitterNode` | interface | `SKEmitterNode.h` | `EmitterNode` |
| `SKFieldNode` | interface | `SKFieldNode.h` | `FieldNode` |
| `SKInterpolationMode` | enum | `SKKeyframeSequence.h` | `InterpolationMode` |
| `SKKeyframeSequence` | interface | `SKKeyframeSequence.h` | `KeyframeSequence` |
| `SKLabelHorizontalAlignmentMode` | enum | `SKLabelNode.h` | `HorizontalAlignmentMode` |
| `SKLabelNode` | interface | `SKLabelNode.h` | `LabelNode` |
| `SKLabelVerticalAlignmentMode` | enum | `SKLabelNode.h` | `VerticalAlignmentMode` |
| `SKLightNode` | interface | `SKLightNode.h` | `LightNode` |
| `SKMutableTexture` | interface | `SKMutableTexture.h` | `MutableTexture` |
| `SKNode` | interface | `SKNode.h` | `Node + NodeExt` |
| `SKNode (NSAccessibility)` | category | `SKNode+NSAccessibility.h` | `NodeExt::{is_accessibility_*, set_accessibility_*, accessibility_label}` |
| `SKParticleRenderOrder` | enum | `SKEmitterNode.h` | `ParticleRenderOrder` |
| `SKPhysicsBody` | interface | `SKPhysicsBody.h` | `PhysicsBody` |
| `SKPhysicsContact` | interface | `SKPhysicsContact.h` | `PhysicsContact` |
| `SKPhysicsContactDelegate` | protocol | `SKPhysicsWorld.h` | `PhysicsContactDelegate + PhysicsWorld::{set_contact_delegate, has_contact_delegate}` |
| `SKPhysicsJoint` | interface | `SKPhysicsJoint.h` | `AsPhysicsJoint + PhysicsJointExt` |
| `SKPhysicsJointFixed` | interface | `SKPhysicsJoint.h` | `PhysicsJointFixed` |
| `SKPhysicsJointLimit` | interface | `SKPhysicsJoint.h` | `PhysicsJointLimit` |
| `SKPhysicsJointPin` | interface | `SKPhysicsJoint.h` | `PhysicsJointPin` |
| `SKPhysicsJointSliding` | interface | `SKPhysicsJoint.h` | `PhysicsJointSliding` |
| `SKPhysicsJointSpring` | interface | `SKPhysicsJoint.h` | `PhysicsJointSpring` |
| `SKPhysicsWorld` | interface | `SKPhysicsWorld.h` | `PhysicsWorld` |
| `SKRange` | interface | `SKConstraint.h` | `ConstraintRange` |
| `SKReachConstraints` | interface | `SKReachConstraints.h` | `ReachConstraints + NodeExt::{reach_constraints, set_reach_constraints}` |
| `SKReferenceNode` | interface | `SKReferenceNode.h` | `ReferenceNode` |
| `SKRegion` | interface | `SKRegion.h` | `Region` |
| `SKRenderer` | interface | `SKRenderer.h` | `Renderer` |
| `SKRepeatMode` | enum | `SKKeyframeSequence.h` | `RepeatMode` |
| `SKScene` | interface | `SKScene.h` | `Scene` |
| `SKSceneDelegate` | protocol | `SKScene.h` | `SceneDelegate + Scene::{set_delegate, has_delegate}` |
| `SKSceneScaleMode` | enum | `SKScene.h` | `SceneScaleMode` |
| `SKShader` | interface | `SKShader.h` | `Shader` |
| `SKShapeNode` | interface | `SKShapeNode.h` | `ShapeNode` |
| `SKSpriteNode` | interface | `SKSpriteNode.h` | `SpriteNode` |
| `SKTexture` | interface | `SKTexture.h` | `Texture` |
| `SKTextureAtlas` | interface | `SKTextureAtlas.h` | `TextureAtlas` |
| `SKTextureFilteringMode` | enum | `SKTexture.h` | `TextureFilteringMode` |
| `SKTileAdjacencyMask` | enum | `SKTileSet.h` | `TileAdjacencyMask` |
| `SKTileDefinition` | interface | `SKTileDefinition.h` | `TileDefinition` |
| `SKTileDefinitionRotation` | enum | `SKTileDefinition.h` | `TileDefinitionRotation` |
| `SKTileGroup` | interface | `SKTileSet.h` | `TileGroup` |
| `SKTileGroupRule` | interface | `SKTileSet.h` | `TileGroupRule` |
| `SKTileMapNode` | interface | `SKTileMapNode.h` | `TileMapNode` |
| `SKTileSet` | interface | `SKTileSet.h` | `TileSet` |
| `SKTileSetType` | enum | `SKTileSet.h` | `TileSetType` |
| `SKTransformNode` | interface | `SKTransformNode.h` | `TransformNode` |
| `SKTransition` | interface | `SKTransition.h` | `Transition + View::present_scene_with_transition` |
| `SKTransitionDirection` | enum | `SKTransition.h` | `TransitionDirection` |
| `SKUniform` | interface | `SKUniform.h` | `Shader::{add_float_uniform, float_uniform_named, remove_uniform_named}` |
| `SKUniformType` | enum | `SKUniform.h` | `UniformType` |
| `SKVideoNode` | interface | `SKVideoNode.h` | `VideoNode` |
| `SKView` | interface | `SKView.h` | `View` |
| `SKViewDelegate` | protocol | `SKView.h` | `ViewDelegate + View::{set_delegate, has_delegate}` |
| `SKWarpGeometry` | interface | `SKWarpGeometry.h` | `WarpGeometry` |
| `SKWarpGeometryGrid` | interface | `SKWarpGeometry.h` | `WarpGeometryGrid` |
| `SKWarpable` | protocol | `SKWarpGeometry.h` | `WarpableNode for SpriteNode/EffectNode` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| — | — | — | No remaining symbol-level coverage gaps. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `UITouch (SKNodeTouches)` | category | `SKNode.h` | iOS-only category with no macOS counterpart | `API_UNAVAILABLE(macos)` |
| `SKNodeFocusBehavior` | enum | `SKNode.h` | macOS-unavailable focus enum | `API_UNAVAILABLE(macos)` |
