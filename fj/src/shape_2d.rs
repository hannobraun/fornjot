use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape2d {
    /// A circle
    Circle(Circle),

    /// A difference between two shapes
    Difference(Box<Difference>),

    /// A rectangle
    Rectangle(Rectangle),

    /// A sketch
    Sketch(Sketch),
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

// TASK: Remove `Rectangle`, once `Shape` is powerful enough to replace it.
/// A rectangle
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Rectangle {
    /// The size of the rectangle along the x-axis
    pub x: f64,

    /// The size of the rectangle along the y-axis
    pub y: f64,
}

impl From<Rectangle> for Shape {
    fn from(shape: Rectangle) -> Self {
        Self::Shape2d(Shape2d::Rectangle(shape))
    }
}

impl From<Rectangle> for Shape2d {
    fn from(shape: Rectangle) -> Self {
        Self::Rectangle(shape)
    }
}

/// A sketch
///
/// Sketches are currently limited to a single cycle of straight lines,
/// represented by a number of points. For example, if the points a, b, and c
/// are provided, the edges ab, bc, and ca are assumed.
#[derive(Clone, Debug)]
#[repr(C)]
// TASK: This is not FFI-safe, meaning it can't really be used in models.
pub struct Sketch(Vec<[f64; 2]>);

impl Sketch {
    /// Create a sketch from a bunch of points
    pub fn from_points(points: Vec<[f64; 2]>) -> Self {
        Self(points)
    }

    /// Return the points of the sketch
    pub fn as_points(&self) -> &[[f64; 2]] {
        self.0.as_slice()
    }
}

impl From<Sketch> for Shape {
    fn from(shape: Sketch) -> Self {
        Self::Shape2d(Shape2d::Sketch(shape))
    }
}

impl From<Sketch> for Shape2d {
    fn from(shape: Sketch) -> Self {
        Self::Sketch(shape)
    }
}
