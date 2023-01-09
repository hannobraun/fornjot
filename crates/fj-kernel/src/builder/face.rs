use crate::{
    objects::Cycle,
    partial::{Partial, PartialCycle, PartialFace},
};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior polygon, from the provided points
    fn add_interior(&mut self) -> Partial<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn add_interior(&mut self) -> Partial<Cycle> {
        let cycle = PartialCycle {
            surface: self.exterior.read().surface.clone(),
            ..Default::default()
        };

        let cycle = Partial::from_partial(cycle);
        self.interiors.push(cycle.clone());

        cycle
    }
}
