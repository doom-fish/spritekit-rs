#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

mod private;

pub mod action;
pub mod attribute;
pub mod audio_node;
pub mod camera_node;
pub mod color;
pub mod constraint;
pub mod crop_node;
pub mod effect_node;
pub mod emitter_node;
pub mod error;
pub mod event;
pub mod ffi;
pub mod field_node;
pub mod keyframe;
pub mod label_node;
pub mod light_node;
pub mod mutable_texture;
pub mod node;
pub mod physics;
pub mod physics_body;
pub mod physics_contact;
pub mod physics_joint;
pub mod physics_world;
pub mod reach_constraints;
pub mod reference_node;
pub mod region;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod shape_node;
pub mod sprite_node;
pub mod texture;
pub mod texture_atlas;
pub mod three_d_node;
pub mod tile;
pub mod transform_node;
pub mod transition;
pub mod video_node;
pub mod view;
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

pub mod prelude {
    pub use crate::{
        read_texture_bytes, Action, ActionTimingMode, AsWarpGeometry, Attribute, AttributeType,
        AttributeValue, AudioNode, BlendMode, CameraNode, Color, Constraint, ConstraintRange,
        CropNode, EffectNode, EmitterNode, Event, FieldNode, HorizontalAlignmentMode,
        InterpolationMode, KeyframeSequence, LabelNode, LightNode, LoadAction, MutableTexture,
        Node, NodeExt, ParticleRenderOrder, PhysicsBody, PhysicsContact, PhysicsContactDelegate,
        PhysicsJointExt, PhysicsJointFixed, PhysicsJointLimit, PhysicsJointPin,
        PhysicsJointSliding, PhysicsJointSpring, PhysicsWorld, ReachConstraints, ReferenceNode,
        Region, RenderPassDescriptor, Renderer, RepeatMode, Scene, SceneDelegate,
        SceneScaleMode, Shader, ShapeNode, SpriteKitError, SpriteNode, StoreAction, Texture,
        TextureAtlas, TextureFilteringMode, ThreeDNode, TileAdjacencyMask, TileDefinition,
        TileDefinitionRotation, TileGroup, TileGroupRule, TileMapNode, TileSet, TileSetType,
        TransformNode, Transition, TransitionDirection, UniformType, VerticalAlignmentMode,
        VideoNode, View, ViewDelegate, WarpGeometry, WarpGeometryGrid, WarpableNode,
    };
}
