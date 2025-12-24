use fj_math::Point;

use crate::store::Store;

#[derive(Default)]
pub struct Geometry {
    pub vertices: Store<Vertex>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Vertex {
    pub point: Point<3>,
}
