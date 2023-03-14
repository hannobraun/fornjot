use crate::{
    objects::{Cycle, Objects},
    partial::{Partial, PartialCycle, PartialFace},
    services::Service,
};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior cycle
    fn add_interior(
        &mut self,
        cycle: PartialCycle,
        objects: &mut Service<Objects>,
    ) -> Partial<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn add_interior(
        &mut self,
        cycle: PartialCycle,
        _: &mut Service<Objects>,
    ) -> Partial<Cycle> {
        let cycle = Partial::from_partial(cycle);
        self.interiors.push(cycle.clone());
        cycle
    }
}
