use std::ops;

use fj_math::Point;

use crate::store::{Index, Store};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Triangle {
    pub vertices: [Index<Vertex>; 3],
}

impl From<[Index<Vertex>; 3]> for Triangle {
    fn from(vertices: [Index<Vertex>; 3]) -> Self {
        Self { vertices }
    }
}

#[derive(Default)]
pub struct Triangles {
    store: Store<Triangle>,
}

impl Triangles {
    #[track_caller]
    pub fn push(
        &mut self,
        triangle: impl Into<Triangle>,
        vertices: &Store<Vertex>,
    ) -> Index<Triangle> {
        let triangle = triangle.into();

        let [a, b, c] = triangle.vertices.map(|v| vertices[v].position);
        if a == b || a == c || b == c {
            panic!("Invalid triangle: {:?}", [a, b, c]);
        }

        self.store.push(triangle)
    }
}

impl ops::Index<Index<Triangle>> for Triangles {
    type Output = Triangle;

    fn index(&self, index: Index<Triangle>) -> &Self::Output {
        &self.store[index]
    }
}
