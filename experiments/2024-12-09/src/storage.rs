use std::marker::PhantomData;

use crate::{
    geometry::Handle,
    math::Plane,
    topology::{Face, Vertex},
};

#[derive(Default)]
pub struct Stores {
    pub faces: Store<Face>,
    pub surfaces: Store<Plane>,
    pub vertices: Store<Vertex>,
}

impl Stores {
    pub fn new() -> Self {
        Self::default()
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
