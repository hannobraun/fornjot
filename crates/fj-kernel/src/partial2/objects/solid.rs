use crate::{
    objects::{Objects, Shell, Solid},
    partial2::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Solid`]
#[derive(Clone, Debug)]
pub struct PartialSolid {
    /// The shells that make up the solid
    pub shells: Vec<Partial<Shell>>,
}

impl PartialSolid {
    /// Construct an instance of `PartialSolid`
    pub fn new(shells: Vec<Partial<Shell>>) -> Self {
        Self { shells }
    }
}

impl PartialObject for PartialSolid {
    type Full = Solid;

    fn from_full(solid: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self::new(
            solid
                .shells()
                .map(|shell| Partial::from_full(shell.clone(), cache))
                .collect(),
        )
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let shells = self.shells.into_iter().map(|shell| shell.build(objects));
        Solid::new(shells)
    }
}

impl Default for PartialSolid {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
