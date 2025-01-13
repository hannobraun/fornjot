use std::marker::PhantomData;

use anymap3::AnyMap;

use crate::geometry::Handle;

#[derive(Default)]
pub struct Stores {
    inner: AnyMap,
}

impl Stores {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<T: 'static>(&mut self) -> &mut Store<T> {
        self.inner.entry::<Store<T>>().or_default()
    }
}

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

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}
