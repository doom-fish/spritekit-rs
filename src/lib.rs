#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

mod private;

/// Module for `SKAction`.
pub mod action;
/// Module for `SKAttribute`, `SKAttributeType`, and `SKAttributeValue`.
pub mod attribute;
/// Module for `SKAudioNode`.
pub mod audio_node;
/// Module for `SKCameraNode`.
pub mod camera_node;
/// Module for `SKColor`.
pub mod color;
/// Module for `SKConstraint` and `SKRange`.
pub mod constraint;
/// Module for `SKCropNode`.
pub mod crop_node;
/// Module for `SKEffectNode`.
pub mod effect_node;
/// Module for `SKEmitterNode`.
pub mod emitter_node;
/// Module for `SpriteKit` bridge failures.
pub mod error;
/// Module for `NSEvent` in `SpriteKit` input handling.
pub mod event;
/// Module for `SpriteKit` bridge symbols.
pub mod ffi;
/// Module for `SKFieldNode`.
pub mod field_node;
/// Module for `SKKeyframeSequence`, `SKInterpolationMode`, and `SKRepeatMode`.
pub mod keyframe;
/// Module for `SKLabelNode`, `SKLabelHorizontalAlignmentMode`, and `SKLabelVerticalAlignmentMode`.
pub mod label_node;
/// Module for `SKLightNode`.
pub mod light_node;
/// Module for `SKMutableTexture`.
pub mod mutable_texture;
/// Module for `SKNode`.
pub mod node;
/// Module for `SKPhysicsBody`, `SKPhysicsContact`, `SKPhysicsJoint`, `SKPhysicsWorld`, and `SKBlendMode`.
pub mod physics;
/// Module for `SKPhysicsBody`.
pub mod physics_body;
/// Module for `SKPhysicsContact`.
pub mod physics_contact;
/// Module for `SKPhysicsJointPin`, `SKPhysicsJointSpring`, `SKPhysicsJointFixed`, `SKPhysicsJointSliding`, and `SKPhysicsJointLimit`.
pub mod physics_joint;
/// Module for `SKPhysicsWorld`.
pub mod physics_world;
/// Module for `SKReachConstraints`.
pub mod reach_constraints;
/// Module for `SKReferenceNode`.
pub mod reference_node;
/// Module for `SKRegion`.
pub mod region;
/// Module for `SKRenderer` and `SKRenderPassDescriptor`.
pub mod renderer;
/// Module for `SKScene` and `SKSceneScaleMode`.
pub mod scene;
/// Module for `SKShader` and `SKUniformType`.
pub mod shader;
/// Module for `SKShapeNode`.
pub mod shape_node;
/// Module for `SKSpriteNode`.
pub mod sprite_node;
/// Module for `SKTexture` and `SKTextureFilteringMode`.
pub mod texture;
/// Module for `SKTextureAtlas`.
pub mod texture_atlas;
/// Module for `SK3DNode`.
pub mod three_d_node;
/// Module for `SpriteKit` tile APIs such as `SKTileDefinition`, `SKTileGroupRule`, `SKTileGroup`, `SKTileSet`, `SKTileMapNode`, and `SKTileAdjacencyMask`.
pub mod tile;
/// Module for `SKTransformNode`.
pub mod transform_node;
/// Module for `SKTransition` and `SKTransitionDirection`.
pub mod transition;
/// Module for `SKVideoNode`.
pub mod video_node;
/// Module for `SKView` and `SKViewDelegate`.
pub mod view;
/// Module for `SKWarpGeometry`, `SKWarpGeometryGrid`, and `SKWarpable`.
pub mod warp;

pub use action::{Action, ActionTimingMode};
pub use attribute::{Attribute, AttributeType, AttributeValue};
pub use audio_node::AudioNode;
pub use camera_node::CameraNode;
pub use color::Color;
pub use constraint::{Constraint, ConstraintRange};
pub use crop_node::CropNode;
pub use effect_node::EffectNode;
pub use emitter_node::{EmitterNode, ParticleRenderOrder};
pub use error::SpriteKitError;
pub use event::Event;
pub use field_node::FieldNode;
pub use keyframe::{InterpolationMode, KeyframeSequence, RepeatMode};
pub use label_node::{HorizontalAlignmentMode, LabelNode, VerticalAlignmentMode};
pub use light_node::LightNode;
pub use mutable_texture::MutableTexture;
pub use node::{Node, NodeExt};
pub use physics::{BlendMode, PhysicsBody, PhysicsContact, PhysicsContactDelegate, PhysicsWorld};
pub use physics_joint::{
    AsPhysicsJoint, PhysicsJointExt, PhysicsJointFixed, PhysicsJointLimit, PhysicsJointPin,
    PhysicsJointSliding, PhysicsJointSpring,
};
pub use reach_constraints::ReachConstraints;
pub use reference_node::ReferenceNode;
pub use region::Region;
pub use renderer::{read_texture_bytes, LoadAction, RenderPassDescriptor, Renderer, StoreAction};
pub use scene::{Scene, SceneDelegate, SceneScaleMode};
pub use shader::{Shader, UniformType};
pub use shape_node::ShapeNode;
pub use sprite_node::SpriteNode;
pub use texture::{Texture, TextureFilteringMode};
pub use texture_atlas::TextureAtlas;
pub use three_d_node::ThreeDNode;
pub use tile::{
    TileAdjacencyMask, TileDefinition, TileDefinitionRotation, TileGroup, TileGroupRule,
    TileMapNode, TileSet, TileSetType,
};
pub use transform_node::TransformNode;
pub use transition::{Transition, TransitionDirection};
pub use video_node::VideoNode;
pub use view::{View, ViewDelegate};
pub use warp::{AsWarpGeometry, WarpGeometry, WarpGeometryGrid, WarpableNode};

pub use apple_cf::cg::{CGPoint, CGRect, CGSize, CGVector};

/// Module for common `SpriteKit` wrapper imports.
pub mod prelude {
    pub use crate::{
        read_texture_bytes, Action, ActionTimingMode, AsWarpGeometry, Attribute, AttributeType,
        AttributeValue, AudioNode, BlendMode, CameraNode, Color, Constraint, ConstraintRange,
        CropNode, EffectNode, EmitterNode, Event, FieldNode, HorizontalAlignmentMode,
        InterpolationMode, KeyframeSequence, LabelNode, LightNode, LoadAction, MutableTexture,
        Node, NodeExt, ParticleRenderOrder, PhysicsBody, PhysicsContact, PhysicsContactDelegate,
        PhysicsJointExt, PhysicsJointFixed, PhysicsJointLimit, PhysicsJointPin,
        PhysicsJointSliding, PhysicsJointSpring, PhysicsWorld, ReachConstraints, ReferenceNode,
        Region, RenderPassDescriptor, Renderer, RepeatMode, Scene, SceneDelegate, SceneScaleMode,
        Shader, ShapeNode, SpriteKitError, SpriteNode, StoreAction, Texture, TextureAtlas,
        TextureFilteringMode, ThreeDNode, TileAdjacencyMask, TileDefinition,
        TileDefinitionRotation, TileGroup, TileGroupRule, TileMapNode, TileSet, TileSetType,
        TransformNode, Transition, TransitionDirection, UniformType, VerticalAlignmentMode,
        VideoNode, View, ViewDelegate, WarpGeometry, WarpGeometryGrid, WarpableNode,
    };
}
