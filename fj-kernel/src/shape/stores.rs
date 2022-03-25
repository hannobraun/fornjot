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
    objects: Vec<Storage<T>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn contains(&self, object: &Handle<T>) -> bool {
        self.objects.contains(object.storage())
    }

    pub fn add(&mut self, object: T) -> Handle<T> {
        let storage = Storage::new(object);
        let handle = storage.handle();

        self.objects.push(storage);

        handle
    }

    pub fn iter(&self) -> Iter<T> {
        // The allocation here is unfortunate, but I think it's fine for now. If
        // this turns into a performance issue, it should be possible to avoid
        // it by adding methods to `Store`, that are geared towards implementing
        // iterators.
        Iter {
            elements: self
                .objects
                .iter()
                .map(|storage| storage.handle())
                .collect(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.objects.iter_mut().map(|storage| storage.get_mut())
    }
}

/// An iterator over geometric or topological objects
///
/// Returned by various methods of the [`Shape`] API.
pub struct Iter<T> {
    elements: Vec<Handle<T>>,
}

impl<T> Iter<T> {
    /// Convert this iterator over handles into an iterator over values
    ///
    /// This is a convenience method, for cases where no `Handle` is needed.
    pub fn values(self) -> impl Iterator<Item = T>
    where
        T: Clone,
    {
        self.elements.into_iter().map(|handle| handle.get())
    }
}

impl<T> Iterator for Iter<T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.pop()
    }
}

pub type IterMut<'r, T> =
    iter::Map<slice::IterMut<'r, Storage<T>>, fn(&mut Storage<T>) -> RefMut<T>>;
