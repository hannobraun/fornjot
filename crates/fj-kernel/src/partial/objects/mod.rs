pub mod cycle;
pub mod edge;
pub mod face;
pub mod vertex;

use crate::{
    objects::{Cycle, Face, GlobalEdge, HalfEdge, Objects, Vertex},
    services::Service,
};

use super::{
    HasPartial, MaybePartial, Partial, PartialCycle, PartialFace,
    PartialGlobalEdge, PartialHalfEdge, PartialVertex,
};

macro_rules! impl_traits {
    ($($full:ty, $partial:ty;)*) => {
        $(
            impl HasPartial for $full {
                type Partial = $partial;
            }

            impl Partial for $partial {
                type Full = $full;

                fn build(self, objects: &mut Service<Objects>) -> Self::Full {
                    self.build(objects)
                }
            }

            impl From<$partial> for MaybePartial<$full> {
                fn from(partial: $partial) -> Self {
                    Self::Partial(partial)
                }
            }
        )*
    };
}

impl_traits!(
    Cycle, PartialCycle;
    Face, PartialFace;
    GlobalEdge, PartialGlobalEdge;
    HalfEdge, PartialHalfEdge;
    Vertex, PartialVertex;
);
