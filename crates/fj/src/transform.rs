use abi_stable::std_types::RBox;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Angle, Shape};

/// A transformed 3-dimensional shape
///
/// # Limitations
///
/// Transformations are currently limited to a rotation, followed by a
/// translation.
///
/// See issue:
/// <https://github.com/hannobraun/Fornjot/issues/101>
#[derive(Clone, Debug, PartialEq, abi_stable::StableAbi)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Transform {
    /// The shape being transformed
    pub shape: Shape,

    /// The axis of the rotation
    pub axis: [f64; 3],

    /// The angle of the rotation
    pub angle: Angle,

    /// The offset of the translation
    pub offset: [f64; 3],
}

impl From<Transform> for Shape {
    fn from(shape: Transform) -> Self {
        Self::Transform(RBox::new(shape))
    }
}
