pub use crate::physics_body::PhysicsBody;
pub use crate::physics_joint::{
    AsPhysicsJoint, PhysicsJointExt, PhysicsJointFixed, PhysicsJointPin, PhysicsJointSliding,
    PhysicsJointSpring,
};
pub use crate::physics_world::PhysicsWorld;

/// `SKBlendMode` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum BlendMode {
    #[default]
    Alpha = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    MultiplyX2 = 4,
    Screen = 5,
    Replace = 6,
    MultiplyAlpha = 7,
}

impl BlendMode {
    #[must_use]
    pub const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Subtract,
            3 => Self::Multiply,
            4 => Self::MultiplyX2,
            5 => Self::Screen,
            6 => Self::Replace,
            7 => Self::MultiplyAlpha,
            _ => Self::Alpha,
        }
    }
}
