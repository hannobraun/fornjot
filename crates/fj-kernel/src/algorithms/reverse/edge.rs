use crate::{
    objects::{HalfEdge, Objects},
    storage::Handle,
    validate::ValidationError,
};

use super::Reverse;

impl Reverse for Handle<HalfEdge> {
    fn reverse(self, objects: &Objects) -> Result<Self, ValidationError> {
        let vertices = {
            let [a, b] = self.vertices().clone();
            [b, a]
        };

        Ok(objects
            .half_edges
            .insert(HalfEdge::new(vertices, self.global_form().clone())))
    }
}
