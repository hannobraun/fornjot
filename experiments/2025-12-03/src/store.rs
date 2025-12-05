use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops, vec,
};

pub struct Store<T> {
    inner: Vec<T>,
}

impl<T> Store<T>
where
    T: PartialEq + fmt::Debug,
{
    pub fn push(&mut self, object: impl Into<T>) -> Index<T> {
        let object = object.into();

        if self.inner.contains(&object) {
            panic!("Duplicate object: {object:?}");
        }

        let index = self.inner.len();
        self.inner.push(object);

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

impl<T> IntoIterator for Store<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
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
