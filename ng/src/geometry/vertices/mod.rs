mod sweep;

pub use self::sweep::SweepVertices;

use crate::math::Point;

/// A shape's vertices
pub trait Vertices {
    type Vertices: IntoIterator<Item = Point>;

    /// Return the shape's vertices
    fn vertices(&self) -> Self::Vertices;
}

impl Vertices for fj::Shape {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        match self {
            Self::Shape2d(shape) => shape.vertices(),
            Self::Shape3d(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape2d {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        match self {
            Self::Circle(shape) => shape.vertices(),
            Self::Square(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape3d {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        match self {
            Self::Sweep(shape) => shape.vertices().into_iter().collect(),
        }
    }
}

impl Vertices for fj::Circle {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        // Circles have just a single round edge with no vertices.
        Vec::new()
    }
}

impl Vertices for fj::Square {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        let s = self.size / 2.;

        #[rustfmt::skip]
        let v = [
            [-s, -s, 0.0],
            [ s, -s, 0.0],
            [ s,  s, 0.0],
            [-s,  s, 0.0],
        ];

        v.map(|coord| coord.into()).to_vec()
    }
}
