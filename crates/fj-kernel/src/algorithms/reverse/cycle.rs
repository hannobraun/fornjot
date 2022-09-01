use crate::objects::Cycle;

use super::Reverse;

impl Reverse for Cycle {
    fn reverse(self) -> Self {
        let surface = *self.surface();

        let mut edges = self
            .into_edges()
            .map(|edge| edge.reverse())
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(surface, edges)
    }
}
