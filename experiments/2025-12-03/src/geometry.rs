use fj_math::Point;

use crate::store::{Index, Store};

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
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

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
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
    pub fn push(
        &mut self,
        triangle: impl Into<Triangle>,
        vertices: &Store<Vertex>,
    ) -> Index<Triangle> {
        let triangle = triangle.into();

        let [a, b, c] = triangle.vertices.map(|v| vertices[v].position);
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);

        self.store.push(triangle)
    }

    pub fn into_store(self) -> Store<Triangle> {
        self.store
    }
}
