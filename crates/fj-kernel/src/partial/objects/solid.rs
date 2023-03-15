use crate::{
    objects::{Objects, Shell, Solid},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
    storage::Handle,
};

/// A partial [`Solid`]
#[derive(Clone, Debug)]
pub struct PartialSolid {
    /// The shells that make up the solid
    pub shells: Vec<Handle<Shell>>,
}

impl PartialObject for PartialSolid {
    type Full = Solid;

    fn new(_: &mut Service<Objects>) -> Self {
        Self { shells: Vec::new() }
    }

    fn from_full(solid: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            shells: solid.shells().cloned().collect(),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        Solid::new(self.shells)
    }
}
