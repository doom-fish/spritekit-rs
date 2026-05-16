# spritekit-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 75
VERIFIED: 35
GAPS: 40
EXEMPT: 0
COVERAGE_PCT: 46.67%

Audit notes:
- Full macOS header pass across `SpriteKit.framework/Headers/*.h`.
- ObjC categories are counted as separate public symbols when they declare macOS-visible API.
- `VERIFIED` means at least one public Rust API reaches the symbol; it does not imply selector-for-selector parity within a large class or category.
- Filtered out macOS-unavailable symbols: `SKNodeFocusBehavior`, `UITouch (SKNodeTouches)`.
- No symbol-level deprecated type/protocol/category declarations were found, so `EXEMPT` is empty.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `SK3DNode` | interface | `SK3DNode.h` | `ThreeDNode` |
| `SKAction` | interface | `SKAction.h` | `Action` |
| `SKAction (SKActions)` | category | `SKAction.h` | `Action::{move_*, rotate_*, scale_*, resize_*, fade_*, set_texture, animate_with_textures, wait, sequence, group, repeat_*}` |
| `SKActionTimingMode` | enum | `SKAction.h` | `ActionTimingMode` |
| `SKAudioNode` | interface | `SKAudioNode.h` | `AudioNode` |
| `SKBlendMode` | enum | `SKNode.h` | `BlendMode` |
| `SKConstraint` | interface | `SKConstraint.h` | `Constraint` |
| `SKEffectNode` | interface | `SKEffectNode.h` | `EffectNode` |
| `SKEmitterNode` | interface | `SKEmitterNode.h` | `EmitterNode` |
| `SKInterpolationMode` | enum | `SKKeyframeSequence.h` | `InterpolationMode` |
| `SKKeyframeSequence` | interface | `SKKeyframeSequence.h` | `KeyframeSequence` |
| `SKLabelHorizontalAlignmentMode` | enum | `SKLabelNode.h` | `HorizontalAlignmentMode` |
| `SKLabelNode` | interface | `SKLabelNode.h` | `LabelNode` |
| `SKLabelVerticalAlignmentMode` | enum | `SKLabelNode.h` | `VerticalAlignmentMode` |
| `SKLightNode` | interface | `SKLightNode.h` | `LightNode` |
| `SKNode` | interface | `SKNode.h` | `Node + NodeExt` |
| `SKPhysicsBody` | interface | `SKPhysicsBody.h` | `PhysicsBody` |
| `SKPhysicsJoint` | interface | `SKPhysicsJoint.h` | `AsPhysicsJoint + PhysicsJointExt` |
| `SKPhysicsJointFixed` | interface | `SKPhysicsJoint.h` | `PhysicsJointFixed` |
| `SKPhysicsJointPin` | interface | `SKPhysicsJoint.h` | `PhysicsJointPin` |
| `SKPhysicsJointSliding` | interface | `SKPhysicsJoint.h` | `PhysicsJointSliding` |
| `SKPhysicsJointSpring` | interface | `SKPhysicsJoint.h` | `PhysicsJointSpring` |
| `SKPhysicsWorld` | interface | `SKPhysicsWorld.h` | `PhysicsWorld` |
| `SKRange` | interface | `SKConstraint.h` | `ConstraintRange` |
| `SKRenderer` | interface | `SKRenderer.h` | `Renderer` |
| `SKRepeatMode` | enum | `SKKeyframeSequence.h` | `RepeatMode` |
| `SKScene` | interface | `SKScene.h` | `Scene` |
| `SKSceneScaleMode` | enum | `SKScene.h` | `SceneScaleMode` |
| `SKShader` | interface | `SKShader.h` | `Shader` |
| `SKSpriteNode` | interface | `SKSpriteNode.h` | `SpriteNode` |
| `SKTexture` | interface | `SKTexture.h` | `Texture` |
| `SKTextureFilteringMode` | enum | `SKTexture.h` | `TextureFilteringMode` |
| `SKUniform` | interface | `SKUniform.h` | `Shader::{add_float_uniform, float_uniform_named, remove_uniform_named}` |
| `SKVideoNode` | interface | `SKVideoNode.h` | `VideoNode` |
| `SKView` | interface | `SKView.h` | `View` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `NSEvent (SKNodeEvent)` | category | `SKNode.h` | The AppKit event-to-node convenience category is not bridged. |
| `SKAction (MixerControl)` | category | `SKAction.h` | No action helpers for audio mixing controls (volume, reverb, obstruction, occlusion). |
| `SKAction (NodeWithPhysicsBody)` | category | `SKAction.h` | No action-based force/impulse helpers; physics is wrapped directly on PhysicsBody instead. |
| `SKAction (PlaybackControl)` | category | `SKAction.h` | No action helpers for play/pause or playback-rate control. |
| `SKAction (SKAudioNode)` | category | `SKAudioNode.h` | No audio-node-specific action helpers are exposed. |
| `SKAction (SKWarpable)` | category | `SKWarpGeometry.h` | No warp-geometry action helpers are exposed. |
| `SKAttribute` | interface | `SKAttribute.h` | No shader attribute descriptor wrapper is exposed. |
| `SKAttributeType` | enum | `SKAttribute.h` | No attribute-type enum wrapper is exposed. |
| `SKAttributeValue` | interface | `SKAttribute.h` | No attribute-value wrapper is exposed; only shader attribute counts are queryable. |
| `SKCameraNode` | interface | `SKCameraNode.h` | Scene camera APIs are not bridged. |
| `SKCropNode` | interface | `SKCropNode.h` | No crop-node wrapper is exposed. |
| `SKFieldNode` | interface | `SKFieldNode.h` | No field-node or force-field APIs are exposed. |
| `SKMutableTexture` | interface | `SKMutableTexture.h` | Mutable texture pixel-generation/update APIs are not wrapped. |
| `SKNode (NSAccessibility)` | category | `SKNode+NSAccessibility.h` | The macOS accessibility category is not bridged. |
| `SKParticleRenderOrder` | enum | `SKEmitterNode.h` | EmitterNode does not expose the particle render-order enum. |
| `SKPhysicsContact` | interface | `SKPhysicsContact.h` | No contact object wrapper is exposed. |
| `SKPhysicsContactDelegate` | protocol | `SKPhysicsWorld.h` | No delegate bridge for physics contact callbacks is exposed. |
| `SKPhysicsJointLimit` | interface | `SKPhysicsJoint.h` | Explicitly skipped in this crate because headless validation currently crashes (see COVERAGE.md). |
| `SKReachConstraints` | interface | `SKReachConstraints.h` | No wrapper is exposed for reach-constraint helpers. |
| `SKReferenceNode` | interface | `SKReferenceNode.h` | No reference-node/archive loading wrapper is exposed. |
| `SKRegion` | interface | `SKRegion.h` | No region wrapper is exposed. |
| `SKSceneDelegate` | protocol | `SKScene.h` | No scene lifecycle delegate bridge is exposed. |
| `SKShapeNode` | interface | `SKShapeNode.h` | No vector shape/path node wrapper is exposed. |
| `SKTextureAtlas` | interface | `SKTextureAtlas.h` | No texture-atlas loading wrapper is exposed. |
| `SKTileAdjacencyMask` | enum | `SKTileSet.h` | Tile adjacency-mask enum is not exposed. |
| `SKTileDefinition` | interface | `SKTileDefinition.h` | Tile definition APIs are not wrapped. |
| `SKTileDefinitionRotation` | enum | `SKTileDefinition.h` | Tile definition rotation enum is not exposed. |
| `SKTileGroup` | interface | `SKTileSet.h` | Tile-group APIs are not wrapped. |
| `SKTileGroupRule` | interface | `SKTileSet.h` | Tile-group-rule APIs are not wrapped. |
| `SKTileMapNode` | interface | `SKTileMapNode.h` | Tile-map node APIs are not wrapped. |
| `SKTileSet` | interface | `SKTileSet.h` | Tile-set APIs are not wrapped. |
| `SKTileSetType` | enum | `SKTileSet.h` | Tile-set type enum is not exposed. |
| `SKTransformNode` | interface | `SKTransformNode.h` | Transform-node APIs are not wrapped; the crate only covers SK3DNode. |
| `SKTransition` | interface | `SKTransition.h` | Scene transition presentation APIs are not wrapped. |
| `SKTransitionDirection` | enum | `SKTransition.h` | Transition direction enum is not exposed. |
| `SKUniformType` | enum | `SKUniform.h` | Shader wrapper only exposes float uniforms, not the uniform-type enum. |
| `SKViewDelegate` | protocol | `SKView.h` | No view-render delegate bridge is exposed. |
| `SKWarpGeometry` | interface | `SKWarpGeometry.h` | Warp geometry types are not exposed. |
| `SKWarpGeometryGrid` | interface | `SKWarpGeometry.h` | Warp geometry grid APIs are not exposed. |
| `SKWarpable` | protocol | `SKWarpGeometry.h` | Warpable protocol requirements are not surfaced on wrapped nodes. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| — | — | — | No symbol-level deprecated SpriteKit declarations were found in the macOS header surface. | — |
