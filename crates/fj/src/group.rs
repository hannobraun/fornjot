#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Shape, Shape3d};

/// A group of two 3-dimensional shapes
///
/// A group is a collection of disjoint shapes. It is not a union, in that the
/// shapes in the group are not allowed to touch or overlap.
///
/// # Limitations
///
/// Whether the shapes in the group touch or overlap is not currently checked.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Group {
    /// The first of the shapes
    pub a: Shape3d,

    /// The second of the shapes
    pub b: Shape3d,
}

impl From<Group> for Shape {
    fn from(shape: Group) -> Self {
        Self::Shape3d(Shape3d::Group(Box::new(shape)))
    }
}

impl From<Group> for Shape3d {
    fn from(shape: Group) -> Self {
        Self::Group(Box::new(shape))
    }
}
