use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape2d {
    /// A square
    Square(Square),
}

/// A square
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Square {
    /// The side length of the square
    pub size: f32,
}

impl From<Square> for Shape {
    fn from(shape: Square) -> Self {
        Self::Shape2d(Shape2d::Square(shape))
    }
}

impl From<Square> for Shape2d {
    fn from(shape: Square) -> Self {
        Self::Square(shape)
    }
}
