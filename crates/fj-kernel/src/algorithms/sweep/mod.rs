//! Sweeping objects along a path to create new objects

mod curve;
mod edge;
mod face;
mod sketch;
mod vertex;

use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    objects::{GlobalVertex, Objects},
    storage::{Handle, ObjectId},
};

/// Sweep an object along a path to create another object
pub trait Sweep: Sized {
    /// The object that is created by sweeping the implementing object
    type Swept;

    /// Sweep the object along the given path
    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        objects: &Objects,
    ) -> Self::Swept {
        let mut cache = SweepCache::default();
        self.sweep_with_cache(path, &mut cache, objects)
    }

    /// Sweep the object along the given path, using the provided cache
    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &Objects,
    ) -> Self::Swept;
}

/// A cache used for sweeping
///
/// See [`Sweep`].
#[derive(Default)]
pub struct SweepCache {
    /// Cache for global vertices
    pub global_vertex: BTreeMap<ObjectId, Handle<GlobalVertex>>,
}
