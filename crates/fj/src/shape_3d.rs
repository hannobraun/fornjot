use crate::{Shape, Shape2d, Transform};
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

/// A sweep of a 2-dimensional shape along straight path
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Sweep {
    /// The 2-dimensional shape being swept
    shape: Shape2d,

    /// The length and direction of the sweep
    path: [f64; 3],
}

impl Sweep {
    /// Create a `Sweep` along a straight path
    pub fn from_path(shape: Shape2d, path: [f64; 3]) -> Self {
        Self { shape, path }
    }

    /// Access the shape being swept
    pub fn shape(&self) -> &Shape2d {
        &self.shape
    }

    /// Access the path of the sweep
    pub fn path(&self) -> [f64; 3] {
        self.path
    }
}

impl From<Sweep> for Shape {
    fn from(shape: Sweep) -> Self {
        Self::Shape3d(shape.into())
    }
}

impl From<Sweep> for Shape3d {
    fn from(shape: Sweep) -> Self {
        Self::Sweep(shape)
    }
}
