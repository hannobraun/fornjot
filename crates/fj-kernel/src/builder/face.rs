use crate::{
    objects::{Cycle, Objects},
    partial::{Partial, PartialFace},
    services::Service,
};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior cycle
    fn add_interior(
        &mut self,
        objects: &mut Service<Objects>,
    ) -> Partial<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn add_interior(&mut self, _: &mut Service<Objects>) -> Partial<Cycle> {
        let cycle = Partial::new();
        self.interiors.push(cycle.clone());
        cycle
    }
}
