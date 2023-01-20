use fj_interop::ext::ArrayExt;

use crate::{
    insert::Insert,
    objects::{HalfEdge, Objects, Vertex},
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<HalfEdge> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let vertices = {
            let [a, b] = self
                .boundary()
                .zip_ext(self.surface_vertices().map(Clone::clone))
                .map(|(point, surface_vertex)| {
                    Vertex::new(point, surface_vertex)
                });
            [b, a]
        };

        HalfEdge::new(
            self.curve().clone(),
            vertices,
            self.global_form().clone(),
        )
        .insert(objects)
    }
}
