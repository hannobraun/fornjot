use crate::{Angle, Shape};

/// A transformed 3-dimensional shape
///
/// # Examples
///
/// Convenient syntax for this operation is available through [`crate::syntax`].
///
/// ``` rust
/// # let shape = fj::Sketch::from_points(vec![[0., 0.], [1., 0.], [0., 1.]]);
/// use fj::syntax::*;
///
/// // `shape` can be anything that converts to `fj::Shape`
/// let rotated = shape.rotate([0., 0., 1.], fj::Angle::from_rev(0.5));
/// let translated = shape.translate([1., 2., 3.]);
/// ```
///
/// # Limitations
///
/// Transformations are currently limited to a rotation, followed by a
/// translation.
///
/// See issue:
/// <https://github.com/hannobraun/Fornjot/issues/101>
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        Self::Transform(Box::new(shape))
    }
}
