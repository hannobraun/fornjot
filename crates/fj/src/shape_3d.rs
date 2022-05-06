use crate::{Shape, Shape2d};

/// A 3-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A difference between two shapes
    Difference(Box<Difference3d>),

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

/// A difference between two shapes
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Difference3d {
    shapes: [Shape3d; 2],
}

impl Difference3d {
    /// Create a `Difference3d` from two shapes
    pub fn from_shapes(shapes: [Shape3d; 2]) -> Self {
        Self { shapes }
    }

    /// Access the shapes that make up the difference
    pub fn shapes(&self) -> &[Shape3d; 2] {
        &self.shapes
    }
}

impl From<Difference3d> for Shape {
    fn from(shape: Difference3d) -> Self {
        Self::Shape3d(shape.into())
    }
}

impl From<Difference3d> for Shape3d {
    fn from(shape: Difference3d) -> Self {
        Self::Difference(Box::new(shape))
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

/// A transformed 3-dimensional shape
///
/// # Limitations
///
/// Transformations are currently limited to a rotation, followed by a
/// translation.
///
/// See issue:
/// <https://github.com/hannobraun/Fornjot/issues/101>
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Transform {
    /// The shape being transformed
    pub shape: Shape3d,

    /// The axis of the rotation
    pub axis: [f64; 3],

    /// The angle of the rotation
    pub angle: f64,

    /// The offset of the translation
    pub offset: [f64; 3],
}

impl From<Transform> for Shape {
    fn from(shape: Transform) -> Self {
        Self::Shape3d(Shape3d::Transform(Box::new(shape)))
    }
}

impl From<Transform> for Shape3d {
    fn from(shape: Transform) -> Self {
        Self::Transform(Box::new(shape))
    }
}

/// A sweep of a 2-dimensional shape along straight path
#[derive(Clone, Debug)]
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
