use crate::objects::{Cycle, Objects};

use super::Reverse;

impl Reverse for Cycle {
    fn reverse(self, objects: &Objects) -> Self {
        let surface = self.surface().clone();

        let mut edges = self
            .into_half_edges()
            .map(|edge| edge.reverse(objects))
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(surface, edges)
    }
}
