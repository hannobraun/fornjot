use std::{iter, slice};

use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{
    handle::{RefMut, Storage},
    Handle,
};

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

    pub fn iter(&self) -> Iter<T> {
        self.inner.iter().map(|storage| storage.handle())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.inner.iter_mut().map(|storage| storage.get_mut())
    }
}

pub type Iter<'r, T> =
    iter::Map<slice::Iter<'r, Storage<T>>, fn(&Storage<T>) -> Handle<T>>;
pub type IterMut<'r, T> =
    iter::Map<slice::IterMut<'r, Storage<T>>, fn(&mut Storage<T>) -> RefMut<T>>;
