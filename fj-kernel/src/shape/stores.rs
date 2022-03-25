use std::slice;

use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{handle::Storage, Handle};

pub type Points = Store<Point<3>>;
pub type Curves = Store<Curve>;
pub type Surfaces = Store<Surface>;

pub type Vertices = Store<Vertex>;
pub type Edges = Store<Edge>;
pub type Cycles = Store<Cycle>;
pub type Faces = Store<Face>;

#[derive(Clone, Debug)]
pub struct Store<T> {
    inner: Vec<Storage<T>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn contains(&self, object: &Handle<T>) -> bool {
        self.inner.contains(object.storage())
    }

    pub fn add(&mut self, object: T) -> Handle<T> {
        let storage = Storage::new(object);
        let handle = storage.handle();

        self.inner.push(storage);

        handle
    }

    pub fn iter(&self) -> slice::Iter<Storage<T>> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Storage<T>> {
        self.inner.iter_mut()
    }
}
