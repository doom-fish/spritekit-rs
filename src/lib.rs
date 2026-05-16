#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

mod private;

pub mod action;
pub mod audio_node;
pub mod color;
pub mod constraint;
pub mod effect_node;
pub mod emitter_node;
pub mod error;
pub mod ffi;
pub mod keyframe;
pub mod label_node;
pub mod light_node;
pub mod node;
pub mod physics;
pub mod physics_body;
pub mod physics_joint;
pub mod physics_world;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod sprite_node;
pub mod texture;
pub mod three_d_node;
pub mod video_node;
pub mod view;

pub use action::{Action, ActionTimingMode};
pub use audio_node::AudioNode;
pub use color::Color;
pub use constraint::{Constraint, ConstraintRange};
pub use effect_node::EffectNode;
pub use emitter_node::EmitterNode;
pub use error::SpriteKitError;
pub use keyframe::{InterpolationMode, KeyframeSequence, RepeatMode};
pub use label_node::{HorizontalAlignmentMode, LabelNode, VerticalAlignmentMode};
pub use light_node::LightNode;
pub use node::{Node, NodeExt};
pub use physics::{BlendMode, PhysicsBody, PhysicsWorld};
pub use physics_joint::{
    AsPhysicsJoint, PhysicsJointExt, PhysicsJointFixed, PhysicsJointPin, PhysicsJointSliding,
    PhysicsJointSpring,
};
pub use renderer::{read_texture_bytes, LoadAction, RenderPassDescriptor, Renderer, StoreAction};
pub use scene::{Scene, SceneScaleMode};
pub use shader::Shader;
pub use sprite_node::SpriteNode;
pub use texture::{Texture, TextureFilteringMode};
pub use three_d_node::ThreeDNode;
pub use video_node::VideoNode;
pub use view::View;

pub use apple_cf::cg::{CGPoint, CGRect, CGSize, CGVector};

pub mod prelude {
    pub use crate::{
        read_texture_bytes, Action, ActionTimingMode, AudioNode, BlendMode, Color, Constraint,
        ConstraintRange, EffectNode, EmitterNode, HorizontalAlignmentMode, InterpolationMode,
        KeyframeSequence, LabelNode, LightNode, LoadAction, Node, NodeExt, PhysicsBody,
        PhysicsJointExt, PhysicsJointFixed, PhysicsJointPin, PhysicsJointSliding,
        PhysicsJointSpring, PhysicsWorld, RenderPassDescriptor, Renderer, RepeatMode, Scene,
        SceneScaleMode, Shader, SpriteKitError, SpriteNode, StoreAction, Texture,
        TextureFilteringMode, ThreeDNode, VerticalAlignmentMode, VideoNode, View,
    };
}
