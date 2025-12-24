use std::ops;

use fj_math::Point;

use crate::store::{Index, Store};

#[derive(Default)]
pub struct Geometry {
    pub points: Store<Point<3>>,
    pub vertices: Store<Vertex>,
    pub triangles: Triangles,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Vertex {
    pub position: Point<3>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Triangle {
    pub vertices: [Index<Vertex>; 3],
}

impl From<[Index<Vertex>; 3]> for Triangle {
    fn from(vertices: [Index<Vertex>; 3]) -> Self {
        let [v0, v1, v2] = vertices;

        Self {
            vertices: [v0, v1, v2],
        }
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
