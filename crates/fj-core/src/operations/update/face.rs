use std::array;

use crate::{
    objects::{Cycle, Face, Region},
    operations::{Polygon, UpdateRegion},
    storage::Handle,
};

/// Update a [`Face`]
pub trait UpdateFace {
    /// Replace the region of the face
    fn update_region(&self, f: impl FnOnce(&Region) -> Handle<Region>) -> Self;

    /// Add the provides interiors to the face
    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self;
}

impl UpdateFace for Face {
    fn update_region(&self, f: impl FnOnce(&Region) -> Handle<Region>) -> Self {
        let region = f(self.region());
        Face::new(self.surface().clone(), region.clone_object())
    }

    fn add_interiors(
        &self,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self {
        let region = self.region().add_interiors(interiors);
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> UpdateFace for Polygon<D> {
    fn update_region(&self, f: impl FnOnce(&Region) -> Handle<Region>) -> Self {
        let face = self.face.update_region(f);
        let edges = array::from_fn(|i| {
            face.region()
                .exterior()
                .nth_half_edge(i)
                .expect("Operation should not have changed length of cycle")
                .clone()
        });
        let vertices = array::from_fn(|i| {
            // The duplicated code here is unfortunate, but unless we get a
            // stable `array::each_ref` and something like `array::unzip`, I'm
            // not sure how to avoid it.
            face.region()
                .exterior()
                .nth_half_edge(i)
                .expect("Operation should not have changed length of cycle")
                .start_vertex()
                .clone()
        });

        Polygon {
            face,
            edges,
            vertices,
        }
    }

    fn add_interiors(
        &self,
        _: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self {
        panic!("Adding interiors to `Polygon` is not supported.")
    }
}
