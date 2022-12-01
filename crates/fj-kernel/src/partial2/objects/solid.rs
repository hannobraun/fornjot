use crate::{
    objects::{Shell, Solid},
    partial2::{Partial, PartialObject},
};

/// A partial [`Solid`]
#[derive(Clone, Default)]
pub struct PartialSolid {
    /// The shells that make up the solid
    pub shells: Vec<Partial<Shell>>,
}

impl PartialObject for PartialSolid {
    type Full = Solid;
}
