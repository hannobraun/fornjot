//! Convenience trait to insert objects into their respective stores
//!
//! See [`Insert`].

use crate::{
    objects::{
        Curve, Cycle, Face, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge,
        Objects, Shell, Sketch, Solid, Surface, SurfaceVertex, Vertex,
    },
    storage::Handle,
    validate::Validate,
};

/// Convenience trait to insert objects into their respective stores
pub trait Insert: Sized + Validate {
    /// Insert the object into its respective store
    fn insert(
        self,
        objects: &Objects,
    ) -> Result<Handle<Self>, <Self as Validate>::Error>;
}

macro_rules! impl_insert {
    ($($ty:ty, $store:ident;)*) => {
        $(
            impl Insert for $ty {
                fn insert(
                    self,
                    objects: &Objects,
                ) -> Result<Handle<Self>, <Self as Validate>::Error> {
                    let handle = objects.$store.reserve();
                    objects.$store.insert(handle.clone(), self)?;
                    Ok(handle)
                }
            }
        )*
    };
}

impl_insert!(
    Curve, curves;
    Cycle, cycles;
    Face, faces;
    GlobalCurve, global_curves;
    GlobalEdge, global_edges;
    GlobalVertex, global_vertices;
    HalfEdge, half_edges;
    Shell, shells;
    Sketch, sketches;
    Solid, solids;
    SurfaceVertex, surface_vertices;
    Surface, surfaces;
    Vertex, vertices;
);
