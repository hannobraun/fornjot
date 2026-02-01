use std::{
    any::type_name,
    fmt::{self, Debug},
    marker::PhantomData,
    ops, vec,
};

#[derive(Debug)]
pub struct Store<T> {
    inner: Vec<T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, object: impl Into<T>) -> Index<T> {
        let object = object.into();

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
        Self::new()
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

#[derive(Eq, Ord, PartialOrd, PartialEq)]
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

impl<T> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = {
            let full_name = type_name::<T>();

            full_name
                .rsplit_once("::")
                .map(|(_, name)| name)
                .unwrap_or(full_name)
        };
        let index = self.inner;

        write!(f, "Index<{type_name}>({index})")
    }
}
