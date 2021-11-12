/// A shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape {
    Shape3d(Shape3d),
}

/// A 2-dimensional shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape2d {
    Square(Square),
}

/// A 3-dimensional shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A cube
    Cube(Cube),
}

impl<T> From<T> for Shape
where
    T: Into<Shape3d>,
{
    fn from(shape: T) -> Self {
        Self::Shape3d(shape.into())
    }
}

/// A square
#[derive(Debug)]
#[repr(C)]
pub struct Square {
    /// The side length of the square
    pub size: f32,
}

impl From<Square> for Shape2d {
    fn from(square: Square) -> Self {
        Self::Square(square)
    }
}

/// A cube
#[derive(Debug)]
#[repr(C)]
pub struct Cube {
    /// The side length of the cube
    pub size: f32,
}

impl From<Cube> for Shape3d {
    fn from(cube: Cube) -> Self {
        Self::Cube(cube)
    }
}
