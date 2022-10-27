use crate::{
    objects::{Cycle, Objects},
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Cycle> {
    fn reverse(self, objects: &Objects) -> Self {
        let surface = self.surface().clone();

        let mut edges = self
            .half_edges()
            .cloned()
            .map(|edge| edge.reverse(objects))
            .collect::<Vec<_>>();

        edges.reverse();

        objects.cycles.insert(Cycle::new(surface, edges))
    }
}
