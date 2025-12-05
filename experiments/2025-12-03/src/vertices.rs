use std::{marker::PhantomData, ops};

use fj_math::Point;

pub struct Store<T> {
    inner: Vec<T>,
}

impl<T> Store<T> {
    pub fn push(&mut self, vertex: impl Into<T>) -> Index<T> {
        let vertex = vertex.into();

        let index = self.inner.len();
        self.inner.push(vertex);

        Index {
            inner: index,
            _t: PhantomData,
        }
    }
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self {
            inner: Vec::default(),
        }
    }
}

impl<T> ops::Index<Index<T>> for Store<T> {
    type Output = T;

    fn index(&self, index: Index<T>) -> &Self::Output {
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
