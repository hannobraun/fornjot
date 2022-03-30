use std::sync::Arc;

use fj_math::Point;
use parking_lot::RwLock;

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

#[derive(Debug)]
pub struct Store<T> {
    objects: Arc<RwLock<Vec<Storage<T>>>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            objects: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn contains(&self, object: &Handle<T>) -> bool {
        self.objects.read().contains(object.storage())
    }

    pub fn add(&mut self, object: T) -> Handle<T> {
        let storage = Storage::new(object);
        let handle = storage.handle();

        self.objects.write().push(storage);

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
                .read()
                .iter()
                .map(|storage| storage.handle())
                .collect(),
        }
    }

    pub fn update<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for storage in self.objects.write().iter_mut() {
            f(&mut storage.get_mut());
        }
    }
}

// Deriving `Clone` would only derive `Clone` where `T: Clone`. This
// implementation doesn't have that limitation, providing `Clone` for all
// `Store`s instead.
impl<T> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.clone(),
        }
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
