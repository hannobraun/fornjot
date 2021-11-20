use crate::math::Point;

/// A shape's vertices
pub trait Vertices {
    /// Return the shape's vertices
    fn vertices(&self) -> Vec<Point>;
}

impl Vertices for fj::Shape {
    fn vertices(&self) -> Vec<Point> {
        match self {
            Self::Shape2d(shape) => shape.vertices(),
            Self::Shape3d(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape2d {
    fn vertices(&self) -> Vec<Point> {
        match self {
            Self::Circle(shape) => shape.vertices(),
            Self::Difference(shape) => shape.vertices(),
            Self::Square(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape3d {
    fn vertices(&self) -> Vec<Point> {
        match self {
            Self::Sweep(shape) => shape.vertices().into_iter().collect(),
        }
    }
}

impl Vertices for fj::Difference {
    fn vertices(&self) -> Vec<Point> {
        // TASK: Implement.
        todo!()
    }
}

impl Vertices for fj::Square {
    fn vertices(&self) -> Vec<Point> {
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

impl Vertices for fj::Sweep {
    fn vertices(&self) -> Vec<Point> {
        // TASK Implement.
        todo!()
    }
}
