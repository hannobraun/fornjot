use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape2d {
    /// A circle
    Circle(Circle),

    /// A difference between two shapes
    Difference(Box<Difference>),

    /// A square
    Square(Rectangle),
}

/// A circle
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Circle {
    /// The radius of the circle
    pub radius: f64,
}

impl From<Circle> for Shape {
    fn from(shape: Circle) -> Self {
        Self::Shape2d(Shape2d::Circle(shape))
    }
}

impl From<Circle> for Shape2d {
    fn from(shape: Circle) -> Self {
        Self::Circle(shape)
    }
}

/// A difference between two shapes
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Difference {
    /// The original shape
    pub a: Shape2d,

    /// The shape being subtracted
    pub b: Shape2d,
}

impl From<Difference> for Shape {
    fn from(shape: Difference) -> Self {
        Self::Shape2d(Shape2d::Difference(Box::new(shape)))
    }
}

impl From<Difference> for Shape2d {
    fn from(shape: Difference) -> Self {
        Self::Difference(Box::new(shape))
    }
}

/// A square
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Rectangle {
    /// The side length of the square
    pub size: f64,
}

impl From<Rectangle> for Shape {
    fn from(shape: Rectangle) -> Self {
        Self::Shape2d(Shape2d::Square(shape))
    }
}

impl From<Rectangle> for Shape2d {
    fn from(shape: Rectangle) -> Self {
        Self::Square(shape)
    }
}
