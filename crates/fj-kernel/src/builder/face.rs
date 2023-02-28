use crate::{
    objects::Cycle,
    partial::{Partial, PartialFace},
};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior cycle
    fn add_interior(&mut self) -> Partial<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn add_interior(&mut self) -> Partial<Cycle> {
        let cycle = Partial::new();
        self.interiors.push(cycle.clone());
        cycle
    }
}
