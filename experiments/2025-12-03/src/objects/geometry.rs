use std::ops;

use fj_math::{Point, Triangle};

use crate::store::{Index, Store};

#[derive(Default)]
pub struct Geometry {
    pub points: Store<Point<3>>,
    pub vertices: Store<Vertex>,
    pub triangles: Triangles,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Vertex {
    pub point: Point<3>,
}

#[derive(Default)]
pub struct Triangles {
    store: Store<Triangle<3>>,
}

impl Triangles {
    #[track_caller]
    pub fn push(
        &mut self,
        triangle: impl Into<Triangle<3>>,
        _: &Store<Point<3>>,
    ) -> Index<Triangle<3>> {
        let triangle = triangle.into();

        let [a, b, c] = triangle.points;
        if a == b || a == c || b == c {
            panic!("Invalid triangle: {:?}", [a, b, c]);
        }

        self.store.push(triangle)
    }
}

impl ops::Index<Index<Triangle<3>>> for Triangles {
    type Output = Triangle<3>;

    fn index(&self, index: Index<Triangle<3>>) -> &Self::Output {
        &self.store[index]
    }
}
