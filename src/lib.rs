#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

mod private;

pub mod action;
pub mod color;
pub mod effect_node;
pub mod error;
pub mod ffi;
pub mod node;
pub mod physics;
pub mod renderer;
pub mod scene;
pub mod sprite_node;
pub mod texture;

pub use action::Action;
pub use color::Color;
pub use effect_node::EffectNode;
pub use error::SpriteKitError;
pub use node::{Node, NodeExt};
pub use physics::{BlendMode, PhysicsBody, PhysicsWorld};
pub use renderer::{read_texture_bytes, LoadAction, RenderPassDescriptor, Renderer, StoreAction};
pub use scene::{Scene, SceneScaleMode};
pub use sprite_node::SpriteNode;
pub use texture::{Texture, TextureFilteringMode};

pub use apple_cf::cg::{CGPoint, CGRect, CGSize, CGVector};

pub mod prelude {
    pub use crate::{
        read_texture_bytes, Action, BlendMode, Color, EffectNode, LoadAction, Node, NodeExt,
        PhysicsBody, PhysicsWorld, RenderPassDescriptor, Renderer, Scene, SceneScaleMode,
        SpriteKitError, SpriteNode, StoreAction, Texture, TextureFilteringMode,
    };
}
