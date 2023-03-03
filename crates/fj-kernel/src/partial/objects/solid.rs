use crate::{
    objects::{Objects, Shell, Solid},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Solid`]
#[derive(Clone, Debug)]
pub struct PartialSolid {
    /// The shells that make up the solid
    pub shells: Vec<Partial<Shell>>,
}

impl PartialObject for PartialSolid {
    type Full = Solid;

    fn new() -> Self {
        Self { shells: Vec::new() }
    }

    fn from_full(solid: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            shells: solid
                .shells()
                .map(|shell| Partial::from_full(shell.clone(), cache))
                .collect(),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let shells = self.shells.into_iter().map(|shell| shell.build(objects));
        Solid::new(shells)
    }
}
