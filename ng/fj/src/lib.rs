/// A shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape {
    Shape3d(Shape3d),
}

impl<T> From<T> for Shape
where
    T: Into<Shape3d>,
{
    fn from(shape: T) -> Self {
        Self::Shape3d(shape.into())
    }
}

/// A 3-dimensional shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A cube
    Cube(Cube),
}

impl From<Cube> for Shape3d {
    fn from(cube: Cube) -> Self {
        Self::Cube(cube)
    }
}

/// A square
#[derive(Debug)]
#[repr(C)]
pub struct Square {
    /// The side length of the square
    pub size: f32,
}

/// A cube
#[derive(Debug)]
#[repr(C)]
pub struct Cube {
    /// The side length of the cube
    pub size: f32,
}
