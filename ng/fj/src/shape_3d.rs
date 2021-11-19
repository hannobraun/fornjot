use crate::{Shape, Shape2d};

/// A 3-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A sweep of 2-dimensional shape along the z-axis
    Sweep(Sweep),
}

impl From<Shape3d> for Shape {
    fn from(shape: Shape3d) -> Self {
        Self::Shape3d(shape.into())
    }
}

/// A sweep of a 2-dimensional shape along the z-axis
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Sweep {
    /// The 2-dimensional shape being swept
    pub shape: Shape2d,

    /// The length of the sweep
    pub length: f32,
}

impl From<Sweep> for Shape {
    fn from(shape: Sweep) -> Self {
        Self::Shape3d(Shape3d::Sweep(shape))
    }
}

impl From<Sweep> for Shape3d {
    fn from(shape: Sweep) -> Self {
        Self::Sweep(shape)
    }
}
