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
}

impl From<[Index<Point<3>>; 3]> for Triangle {
    fn from(vertices: [Index<Point<3>>; 3]) -> Self {
        let [p0, p1, p2] = vertices;

        Self {
            points: [p0, p1, p2],
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
        points: &Store<Point<3>>,
    ) -> Index<Triangle> {
        let triangle = triangle.into();

        let [a, b, c] = triangle.points.map(|p| points[p]);
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
