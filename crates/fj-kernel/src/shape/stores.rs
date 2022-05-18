use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

use anymap::AnyMap;
use fj_math::Point;
use parking_lot::{RwLock, RwLockReadGuard};
use slotmap::{DefaultKey, SlotMap};

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::Object;

#[derive(Clone, Debug)]
pub struct Stores {
    pub points: Store<Point<3>>,
    pub curves: Store<Curve<3>>,
    pub surfaces: Store<Surface>,

    pub vertices: Store<Vertex<3>>,
    pub edges: Store<Edge<3>>,
    pub cycles: Store<Cycle<3>>,
    pub faces: Store<Face>,
}

impl Stores {
    pub fn get<T: Object>(&self) -> Store<T> {
        let mut stores = AnyMap::new();

        stores.insert(self.points.clone());
        stores.insert(self.curves.clone());
        stores.insert(self.surfaces.clone());

        stores.insert(self.vertices.clone());
        stores.insert(self.edges.clone());
        stores.insert(self.cycles.clone());
        stores.insert(self.faces.clone());

        stores
            .remove::<Store<T>>()
            // Can't panic, as `T` is bound by `Object`, and we added the stores
            // for all types of objects above.
            .expect("Invalid object type")
    }
}

#[derive(Debug)]
pub struct Store<T: Object> {
    objects: Arc<RwLock<Objects<T>>>,
}

impl<T: Object> Store<T> {
    pub fn new() -> Self {
        Self {
            objects: Arc::new(RwLock::new(SlotMap::new())),
        }
    }

    pub fn insert(&mut self, object: T) -> Handle<T> {
        let key = self.objects.write().insert(object);
        Handle::new(key, self.clone())
    }

    pub fn contains(&self, object: &Handle<T>) -> bool {
        object.store() == self && self.objects.read().contains_key(object.key())
    }

    pub fn read(&self) -> RwLockReadGuard<Objects<T>> {
        self.objects.read()
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
                .map(|(key, _)| Handle::new(key, self.clone()))
                .collect(),
        }
    }

    pub fn update<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for (_, object) in self.objects.write().iter_mut() {
            f(object);
        }
    }

    fn ptr(&self) -> *const () {
        Arc::as_ptr(&self.objects) as _
    }
}

// Deriving `Clone` would only derive `Clone` where `T: Clone`. This
// implementation doesn't have that limitation, providing `Clone` for all
// `Store`s instead.
impl<T: Object> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.clone(),
        }
    }
}

impl<T: Object> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Object> PartialEq for Store<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr().eq(&other.ptr())
    }
}

impl<T: Object> Eq for Store<T> {}

impl<T: Object> Ord for Store<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ptr().cmp(&other.ptr())
    }
}

impl<T: Object> PartialOrd for Store<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Object> Hash for Store<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr().hash(state);
    }
}

pub type Objects<T> = SlotMap<DefaultKey, T>;

/// A handle to an object stored within [`Shape`]
///
/// If an object of type `T` (this could be `Curve`, `Vertex`, etc.) is added to
/// `Shape`, a `Handle<T>` is returned. This handle is then used in topological
/// types to refer to the object, instead of having those types own an instance
/// of the object.
///
/// This approach has two advantages:
///
/// 1. The object can't be mutated through the handle. Since an object can be
///    referred to by multiple other objects, mutating it locally would have no
///    effect on those other references. `Handle` preventing that removes this
///    source of errors.
/// 2. The object is guaranteed to be in `Shape`, as `Handle`s can't be created
///    any other way. This means that if the `Shape` needs to be modified, any
///    objects can be updated once, without requiring an update of all the other
///    objects that reference it.
///
/// # Equality
///
/// The equality of [`Handle`] is very strictly defined in terms of identity.
/// Two [`Handle`]s are considered equal, if they refer to objects in the same
/// memory location.
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Handle<T: Object> {
    key: DefaultKey,
    store: Store<T>,
}

impl<T: Object> Handle<T> {
    pub(super) fn new(key: DefaultKey, store: Store<T>) -> Self {
        Self { key, store }
    }

    pub(super) fn key(&self) -> DefaultKey {
        self.key
    }

    pub(super) fn store(&self) -> &Store<T> {
        &self.store
    }

    /// Access the object that the handle references
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.store
            .read()
            .get(self.key)
            // Can't panic, unless the handle was invalid in the first place.
            // Objects are never removed from `Store`, so if we have a handle
            // pointing to it, it should be there.
            .unwrap()
            .clone()
    }
}

impl<T: Object> fmt::Debug for Handle<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handle => {:?}", self.get())
    }
}

/// An iterator over geometric or topological objects
///
/// Returned by various methods of the [`Shape`] API.
pub struct Iter<T: Object> {
    elements: Vec<Handle<T>>,
}

impl<T: Object> Iter<T> {
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

impl<T: Object> Iterator for Iter<T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.pop()
    }
}
