pub mod curve;
pub mod cycle;
pub mod edge;
pub mod vertex;

use crate::{
    objects::{
        Curve, Cycle, GlobalEdge, GlobalVertex, HalfEdge, Stores,
        SurfaceVertex, Vertex,
    },
    stores::Handle,
};

use super::{
    HasPartial, MaybePartial, Partial, PartialCurve, PartialCycle,
    PartialGlobalEdge, PartialGlobalVertex, PartialHalfEdge,
    PartialSurfaceVertex, PartialVertex,
};

macro_rules! impl_traits {
    ($($full:ty, $partial:ty;)*) => {
        $(
            impl HasPartial for $full {
                type Partial = $partial;
            }

            impl Partial for $partial {
                type Full = $full;

                fn build(self, stores: &Stores) -> Self::Full {
                    self.build(stores)
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
    Handle<Curve>, PartialCurve;
    Cycle, PartialCycle;
    GlobalEdge, PartialGlobalEdge;
    Handle<GlobalVertex>, PartialGlobalVertex;
    HalfEdge, PartialHalfEdge;
    SurfaceVertex, PartialSurfaceVertex;
    Vertex, PartialVertex;
);
