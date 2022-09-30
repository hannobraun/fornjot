use crate::objects::Cycle;

use super::Reverse;

impl Reverse for Cycle {
    fn reverse(self) -> Self {
        let surface = self.surface().clone();

        let mut edges = self
            .into_half_edges()
            .map(|edge| edge.reverse())
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(surface, edges)
    }
}
