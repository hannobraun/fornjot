use std::{marker::PhantomData, ops};

use fj_math::Point;

#[derive(Default)]
pub struct Vertices {
    inner: Vec<Vertex>,
}

impl Vertices {
    pub fn push(&mut self, position: impl Into<Point<3>>) -> Index<Vertex> {
        let position = position.into();

        let index = self.inner.len();
        self.inner.push(Vertex { position });

        Index {
            inner: index,
            _t: PhantomData,
        }
    }
}

impl ops::Index<Index<Vertex>> for Vertices {
    type Output = Vertex;

    fn index(&self, index: Index<Vertex>) -> &Self::Output {
        &self.inner[index.inner]
    }
}

pub struct Vertex {
    pub position: Point<3>,
}

pub struct Index<T> {
    inner: usize,
    _t: PhantomData<T>,
}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Index<T> {}
