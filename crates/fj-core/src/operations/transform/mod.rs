//! API for transforming objects

mod curve;
mod cycle;
mod face;
mod half_edge;
mod region;
mod shell;
mod solid;
mod surface;
mod vertex;

use std::collections::{BTreeMap, btree_map};

use fj_math::{Transform, Vector};
use type_map::TypeMap;

use crate::{
    Core,
    operations::insert::Insert,
    storage::{Handle, ObjectId},
    topology::{AnyObject, Stored},
};

use super::derive::DeriveFrom;

/// Transform an object
///
/// # Implementation Note
///
/// So far, a general `transform` method is available, along some convenience
/// methods for more specific transformations.
///
/// More convenience methods can be added as required. The only reason this
/// hasn't been done so far, is that no one has put in the work yet.
pub trait TransformObject: Sized {
    /// The result of the transformation
    type Transformed;

    /// Transform the object
    fn transform(
        self,
        transform: &Transform,
        core: &mut Core,
    ) -> Self::Transformed {
        let mut cache = TransformCache::default();
        self.transform_with_cache(transform, core, &mut cache)
    }

    /// Transform the object using the provided cache
    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed;

    /// Translate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn translate(
        self,
        offset: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Self::Transformed {
        self.transform(&Transform::translation(offset), core)
    }

    /// Rotate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn rotate(
        self,
        axis_angle: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Self::Transformed {
        self.transform(&Transform::rotation(axis_angle), core)
    }
}

impl<T> TransformObject for Handle<T>
where
    T: Clone
        + Insert<Inserted = Handle<T>>
        + TransformObject<Transformed = T>
        + 'static,
    Handle<T>: Into<AnyObject<Stored>>,
{
    type Transformed = Self;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        if let Some(object) = cache.get(&self) {
            return object.clone();
        }

        let transformed = self
            .clone_object()
            .transform_with_cache(transform, core, cache)
            .insert(core)
            .derive_from(&self, core);

        cache.insert(self.clone(), transformed.clone());

        transformed
    }
}

/// A cache for transformed objects
///
/// See [`TransformObject`].
#[derive(Default)]
pub struct TransformCache(TypeMap);

impl TransformCache {
    fn entry<T: 'static>(
        &mut self,
        key: &Handle<T>,
    ) -> btree_map::Entry<'_, ObjectId, Handle<T>> {
        let map = self
            .0
            .entry::<BTreeMap<ObjectId, Handle<T>>>()
            .or_insert_with(BTreeMap::new);

        map.entry(key.id())
    }

    fn get<T: 'static>(&mut self, key: &Handle<T>) -> Option<&Handle<T>> {
        let map = self
            .0
            .entry::<BTreeMap<ObjectId, Handle<T>>>()
            .or_insert_with(BTreeMap::new);

        map.get(&key.id())
    }

    fn insert<T: 'static>(&mut self, key: Handle<T>, value: Handle<T>) {
        let map = self
            .0
            .entry::<BTreeMap<ObjectId, Handle<T>>>()
            .or_insert_with(BTreeMap::new);

        map.insert(key.id(), value);
    }
}
