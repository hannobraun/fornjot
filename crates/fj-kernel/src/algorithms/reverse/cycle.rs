use crate::{
    objects::{Cycle, Objects},
    storage::Handle,
    validate::ValidationError,
};

use super::Reverse;

impl Reverse for Handle<Cycle> {
    fn reverse(self, objects: &Objects) -> Result<Self, ValidationError> {
        let surface = self.surface().clone();

        let mut edges = self
            .half_edges()
            .cloned()
            .map(|edge| edge.reverse(objects))
            .collect::<Result<Vec<_>, _>>()?;

        edges.reverse();

        Ok(objects.cycles.insert(Cycle::new(surface, edges))?)
    }
}
