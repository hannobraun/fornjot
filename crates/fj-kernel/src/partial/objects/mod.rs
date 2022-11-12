pub mod curve;
pub mod cycle;
pub mod edge;
pub mod face;
pub mod surface;
pub mod vertex;

use crate::objects::{
    Curve, Cycle, Face, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge,
    Objects, Surface, SurfaceVertex, Vertex,
};

use super::{
    HasPartial, MaybePartial, Partial, PartialCurve, PartialCycle, PartialFace,
    PartialGlobalCurve, PartialGlobalEdge, PartialGlobalVertex,
    PartialHalfEdge, PartialSurface, PartialSurfaceVertex, PartialVertex,
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
                        Self::Full,
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
    Face, PartialFace;
    GlobalCurve, PartialGlobalCurve;
    GlobalEdge, PartialGlobalEdge;
    GlobalVertex, PartialGlobalVertex;
    HalfEdge, PartialHalfEdge;
    Surface, PartialSurface;
    SurfaceVertex, PartialSurfaceVertex;
    Vertex, PartialVertex;
);
