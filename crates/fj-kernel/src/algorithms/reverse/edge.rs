use crate::{
    insert::Insert,
    objects::{HalfEdge, Objects},
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<HalfEdge> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let boundary = {
            let [a, b] = self.boundary();
            [b, a]
        };
        let surface_vertices = {
            let [a, b] = self.surface_vertices().map(Clone::clone);
            [b, a]
        };

        HalfEdge::new(
            self.curve(),
            boundary,
            surface_vertices,
            self.global_form().clone(),
        )
        .insert(objects)
    }
}
