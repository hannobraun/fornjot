//! API for transforming objects

mod cycle;
mod edge;
mod face;
mod set;
mod shell;
mod solid;
mod surface;
mod vertex;

use std::collections::BTreeMap;

use fj_math::{Transform, Vector};
use type_map::TypeMap;

use crate::{
    operations::Insert,
    services::Services,
    storage::{Handle, ObjectId},
};

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
    /// Transform the object
    fn transform(self, transform: &Transform, services: &mut Services) -> Self {
        let mut cache = TransformCache::default();
        self.transform_with_cache(transform, services, &mut cache)
    }

    /// Transform the object using the provided cache
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self;

    /// Translate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn translate(
        self,
        offset: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Self {
        self.transform(&Transform::translation(offset), services)
    }

    /// Rotate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn rotate(
        self,
        axis_angle: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Self {
        self.transform(&Transform::rotation(axis_angle), services)
    }
}

impl<T> TransformObject for Handle<T>
where
    T: Clone + Insert + TransformObject + 'static,
{
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        if let Some(object) = cache.get(&self) {
            return object.clone();
        }

        let transformed = self
            .clone_object()
            .transform_with_cache(transform, services, cache)
            .insert(services);

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
