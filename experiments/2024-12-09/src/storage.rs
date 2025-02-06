use std::marker::PhantomData;

use crate::{
    math::Plane,
    topology::{face::Face, vertex::Vertex},
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
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}
