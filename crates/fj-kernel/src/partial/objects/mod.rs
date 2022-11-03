pub mod curve;
pub mod cycle;
pub mod edge;
pub mod vertex;

use crate::{
    objects::{
        Curve, Cycle, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        SurfaceVertex, Vertex,
    },
    storage::Handle,
};

use super::{
    HasPartial, MaybePartial, Partial, PartialCurve, PartialCycle,
    PartialGlobalCurve, PartialGlobalEdge, PartialGlobalVertex,
    PartialHalfEdge, PartialSurfaceVertex, PartialVertex,
};

macro_rules! impl_traits {
    ($($full:ty, $partial:ty;)*) => {
        $(
            impl HasPartial for $full {
                type Partial = $partial;
            }

            impl Partial for $partial {
                type Full = $full;

                fn build(self, objects: &Objects)
                    -> Result<
                        Handle<Self::Full>,
                        crate::validate::ValidationError
                    >
                {
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
    Curve, PartialCurve;
    Cycle, PartialCycle;
    GlobalCurve, PartialGlobalCurve;
    GlobalEdge, PartialGlobalEdge;
    GlobalVertex, PartialGlobalVertex;
    HalfEdge, PartialHalfEdge;
    SurfaceVertex, PartialSurfaceVertex;
    Vertex, PartialVertex;
);
