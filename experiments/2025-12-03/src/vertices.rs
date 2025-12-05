use std::{marker::PhantomData, ops};

use fj_math::Point;

#[derive(Default)]
pub struct Store<T> {
    inner: Vec<T>,
}

impl Store<Vertex> {
    pub fn push(&mut self, vertex: impl Into<Vertex>) -> Index<Vertex> {
        let vertex = vertex.into();

        let index = self.inner.len();
        self.inner.push(vertex);

        Index {
            inner: index,
            _t: PhantomData,
        }
    }
}

impl Default for Store<Vertex> {
    fn default() -> Self {
        Self {
            inner: Vec::default(),
        }
    }
}

impl ops::Index<Index<Vertex>> for Store<Vertex> {
    type Output = Vertex;

    fn index(&self, index: Index<Vertex>) -> &Self::Output {
        &self.inner[index.inner]
    }
}

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
