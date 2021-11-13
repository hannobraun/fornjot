mod sweep;

use crate::math::Point;

/// Return a shape's vertices
pub trait Vertices {
    type Vertices: IntoIterator<Item = Point>;

    /// Return a shape's vertices
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
            Self::Square(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape3d {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        match self {
            Self::Cube(shape) => shape.vertices(),
            Self::Sweep(shape) => shape.vertices(),
        }
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

impl Vertices for fj::Cube {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        let s = self.size / 2.;

        #[rustfmt::skip]
        let v = [
            [-s, -s, -s],
            [-s, -s,  s],
            [-s,  s, -s],
            [-s,  s,  s],
            [ s, -s, -s],
            [ s, -s,  s],
            [ s,  s, -s],
            [ s,  s,  s],
        ];

        v.map(|coord| coord.into()).to_vec()
    }
}
