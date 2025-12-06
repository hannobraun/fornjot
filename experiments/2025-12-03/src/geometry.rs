use fj_math::Point;

use crate::store::Index;

#[derive(Debug, Eq, PartialEq)]
pub struct Vertex {
    pub position: Point<3>,
}

impl From<[f64; 3]> for Vertex {
    fn from(position: [f64; 3]) -> Self {
        let position = position.into();
        Self { position }
    }
}

impl From<Point<3>> for Vertex {
    fn from(position: Point<3>) -> Self {
        Self { position }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Triangle {
    pub vertices: [Index<Vertex>; 3],
}

impl From<[Index<Vertex>; 3]> for Triangle {
    fn from(vertices: [Index<Vertex>; 3]) -> Self {
        Self { vertices }
    }
}
