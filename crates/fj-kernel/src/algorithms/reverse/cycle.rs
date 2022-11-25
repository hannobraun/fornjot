use crate::{
    insert::Insert,
    objects::{Cycle, Objects},
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Cycle> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let mut edges = self
            .half_edges()
            .cloned()
            .map(|edge| edge.reverse(objects))
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges).insert(objects)
    }
}
