use fj_interop::ext::ArrayExt;

use crate::{
    insert::Insert,
    objects::{HalfEdge, Objects},
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<HalfEdge> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let vertices = {
            let [a, b] = self
                .boundary()
                .zip_ext(self.surface_vertices().map(Clone::clone));
            [b, a]
        };

        HalfEdge::new(self.curve(), vertices, self.global_form().clone())
            .insert(objects)
    }
}
