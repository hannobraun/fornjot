use crate::{Shape, Shape2d};

/// A 3-dimensional shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A cube
    Cube(Cube),

    /// A sweep of 2-dimensional shape along the z-axis
    Sweep(Sweep),
}

impl From<Shape3d> for Shape {
    fn from(shape: Shape3d) -> Self {
        Self::Shape3d(shape.into())
    }
}

/// A cube
#[derive(Debug)]
#[repr(C)]
pub struct Cube {
    /// The side length of the cube
    pub size: f32,
}

impl From<Cube> for Shape {
    fn from(shape: Cube) -> Self {
        Self::Shape3d(Shape3d::Cube(shape))
    }
}

impl From<Cube> for Shape3d {
    fn from(shape: Cube) -> Self {
        Self::Cube(shape)
    }
}

/// A sweep of 2-dimensional shape along the z-axis
#[derive(Debug)]
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
