use crate::Shape;

/// A group of two 3-dimensional shapes
///
/// A group is a collection of disjoint shapes. It is not a union, in that the
/// shapes in the group are not allowed to touch or overlap.
///
/// # Examples
///
/// Convenient syntax for this operation is available through [`crate::syntax`].
///
/// ``` rust
/// # let a = fj::Sketch::from_points(vec![[0., 0.], [1., 0.], [0., 1.]]);
/// # let b = fj::Sketch::from_points(vec![[2., 0.], [3., 0.], [2., 1.]]);
/// use fj::syntax::*;
///
/// // `a` and `b` can be anything that converts to `fj::Shape`
/// let group = a.group(&b);
/// ```
///
/// # Limitations
///
/// Whether the shapes in the group touch or overlap is not currently checked.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Group {
    /// The first of the shapes
    pub a: Shape,

    /// The second of the shapes
    pub b: Shape,
}

impl From<Group> for Shape {
    fn from(shape: Group) -> Self {
        Self::Group(Box::new(shape))
    }
}
