use crate::{
    insert::Insert,
    objects::{Cycle, Objects},
    partial::PartialFace,
    services::Service,
    storage::Handle,
};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior cycle
    fn add_interior(
        &mut self,
        cycle: Cycle,
        objects: &mut Service<Objects>,
    ) -> Handle<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn add_interior(
        &mut self,
        cycle: Cycle,
        objects: &mut Service<Objects>,
    ) -> Handle<Cycle> {
        let cycle = cycle.insert(objects);
        self.interiors.push(cycle.clone());
        cycle
    }
}
