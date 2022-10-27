use crate::{
    objects::{HalfEdge, Objects},
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<HalfEdge> {
    fn reverse(self, objects: &Objects) -> Self {
        let vertices = {
            let [a, b] = self.vertices().clone();
            [b, a]
        };

        objects
            .half_edges
            .insert(HalfEdge::new(vertices, self.global_form().clone()))
    }
}
