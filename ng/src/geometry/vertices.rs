use crate::math::Point;

/// Return a shape's vertices
pub trait Vertices {
    /// Return a shape's vertices
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
            Self::Square(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape3d {
    fn vertices(&self) -> Vec<Point> {
        match self {
            Self::Cube(shape) => shape.vertices(),
            Self::Sweep(_) => {
                // TASK: Implement.
                todo!()
            }
        }
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

impl Vertices for fj::Cube {
    fn vertices(&self) -> Vec<Point> {
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
