use std::{hash::Hash, marker::PhantomData};

/// The geometry cache
///
/// Due to floating point accuracy issues, it is error-prone to refer to
/// geometry directly.
///
/// Since any object can be referenced by multiple other objects (for example, a
/// vertex can be shared by multiple edges), storing such a reference as
/// geometry (for example, storing a vertex as a point in space) risks computing
/// those same objects in different ways, leading to different results.
///
/// This can result in the same objects being mistaken for different ones, due
/// to slight differences.
///
/// This cache presents a principled approach to preventing this: Each geometric
/// object is computed only once, and is only ever referred to by the handle
/// returned from this cache.
///
/// The alternative would be to be really careful, everywhere, and plug holes as
/// they are found.
pub struct Cache;

impl Cache {
    /// Construct a new instance of the geometry cache
    pub fn new() -> Self {
        Self
    }
}

/// An handle that refers to a geometric object
///
/// Instances of this struct are constructed when an object is added to
/// [`Cache`]. It can afterwards be used to retrieved the geometrical
/// representation of that object from the cache.
///
/// This struct must be the only way that objects are referenced. See the
/// documentation of [`Cache`] for more information.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Handle<T>(u64, PhantomData<T>);

impl<T> Eq for Handle<T> where T: PartialEq {}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
