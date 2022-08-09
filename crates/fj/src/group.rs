use abi_stable::std_types::RBox;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Shape;

/// A group of two 3-dimensional shapes
///
/// A group is a collection of disjoint shapes. It is not a union, in that the
/// shapes in the group are not allowed to touch or overlap.
///
/// # Limitations
///
/// Whether the shapes in the group touch or overlap is not currently checked.
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Group {
    /// The first of the shapes
    pub a: Shape,

    /// The second of the shapes
    pub b: Shape,
}

impl From<Group> for Shape {
    fn from(shape: Group) -> Self {
        Self::Group(RBox::new(shape))
    }
}
