use crate::Shape;

/// A 2-dimensional shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape2d {
    Square(Square),
}

/// A square
#[derive(Debug)]
#[repr(C)]
pub struct Square {
    /// The side length of the square
    pub size: f32,
}

impl From<Square> for Shape {
    fn from(square: Square) -> Self {
        Self::Shape2d(Shape2d::Square(square))
    }
}

impl From<Square> for Shape2d {
    fn from(shape: Square) -> Self {
        Self::Square(shape)
    }
}
