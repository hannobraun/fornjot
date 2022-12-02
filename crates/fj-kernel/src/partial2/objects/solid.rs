use crate::{
    objects::{Objects, Shell, Solid},
    partial2::{Partial, PartialObject},
    services::Service,
};

/// A partial [`Solid`]
#[derive(Clone, Default)]
pub struct PartialSolid {
    /// The shells that make up the solid
    pub shells: Vec<Partial<Shell>>,
}

impl PartialObject for PartialSolid {
    type Full = Solid;

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let shells = self.shells.into_iter().map(|shell| shell.build(objects));
        Solid::new(shells)
    }
}
