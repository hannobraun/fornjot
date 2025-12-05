use std::{marker::PhantomData, ops};

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
