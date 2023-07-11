use std::array;

use crate::{
    objects::{Face, Region},
    operations::Polygon,
    storage::Handle,
};

/// Update a [`Face`]
pub trait UpdateFace {
    /// Replace the region of the face
    #[must_use]
    fn update_region(
        &self,
        f: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self;
}

impl UpdateFace for Face {
    fn update_region(
        &self,
        f: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self {
        let region = f(self.region());
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> UpdateFace for Polygon<D> {
    fn update_region(
        &self,
        f: impl FnOnce(&Handle<Region>) -> Handle<Region>,
    ) -> Self {
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
}
