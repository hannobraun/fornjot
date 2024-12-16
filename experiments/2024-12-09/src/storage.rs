use std::marker::PhantomData;

use crate::geometry::Handle;

pub struct Store<T> {
    _t: PhantomData<T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self { _t: PhantomData }
    }

    pub fn insert(&mut self, op: T) -> Handle<T> {
        Handle::new(op)
    }
}
