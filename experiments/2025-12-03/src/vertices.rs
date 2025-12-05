use std::ops;

use fj_math::Point;

#[derive(Default)]
pub struct Vertices {
    inner: Vec<Vertex>,
}

impl Vertices {
    pub fn push(&mut self, position: impl Into<Point<3>>) -> usize {
        let position = position.into();

        let index = self.inner.len();
        self.inner.push(Vertex { position });

        index
    }
}

impl ops::Index<usize> for Vertices {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

pub struct Vertex {
    pub position: Point<3>,
}
