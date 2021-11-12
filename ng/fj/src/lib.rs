mod shape_2d;

pub use self::shape_2d::*;

/// A shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape {
    Shape2d(Shape2d),
    Shape3d(Shape3d),
}

/// A 3-dimensional shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A cube
    Cube(Cube),
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
