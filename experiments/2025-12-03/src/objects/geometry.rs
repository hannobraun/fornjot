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
    pub point: Index<Point<3>>,
    pub position: Point<3>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Triangle {
    pub points: [Index<Point<3>>; 3],
    pub vertices: [Index<Vertex>; 3],
}

impl From<[(Index<Point<3>>, Index<Vertex>); 3]> for Triangle {
    fn from(vertices: [(Index<Point<3>>, Index<Vertex>); 3]) -> Self {
        let [p0, p1, p2] = vertices.map(|(p, _)| p);
        let [v0, v1, v2] = vertices.map(|(_, v)| v);

        Self {
            points: [p0, p1, p2],
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
        _: &Store<Point<3>>,
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
