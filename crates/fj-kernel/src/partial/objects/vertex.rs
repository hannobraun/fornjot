use crate::{
    objects::{Objects, Vertex},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
};

/// A partial [`Vertex`]
#[derive(Clone, Debug)]
pub struct PartialVertex {}

impl PartialObject for PartialVertex {
    type Full = Vertex;

    fn new(_: &mut Service<Objects>) -> Self {
        Self {}
    }

    fn from_full(_: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {}
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        Vertex::new()
    }
}
