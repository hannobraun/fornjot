use crate::{Group, Shape, Sweep, Transform};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A 3-dimensional shape
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Shape3d {
    /// A group of two 3-dimensional shapes
    Group(Box<Group>),

    /// A sweep of 2-dimensional shape along the z-axis
    Sweep(Sweep),

    /// A transformed 3-dimensional shape
    Transform(Box<Transform>),
}

impl From<Shape3d> for Shape {
    fn from(shape: Shape3d) -> Self {
        Self::Shape3d(shape)
    }
}
