use crate::{
    objects::{
        Cycle, Face, GlobalEdge, HalfEdge, Objects, Shell, Sketch, Solid,
        Surface, Vertex,
    },
    services::{Service, ServiceObjectsExt},
    storage::Handle,
};

/// Insert an object into its respective store
///
/// This is the only primitive operation that is directly understood by
/// `Service<Objects>`. All other operations are built on top of it.
pub trait Insert: Sized {
    /// Insert the object into its respective store
    fn insert(self, objects: &mut Service<Objects>) -> Handle<Self>;
}

macro_rules! impl_insert {
    ($($ty:ty, $store:ident;)*) => {
        $(
            impl Insert for $ty {
                fn insert(self, objects: &mut Service<Objects>) -> Handle<Self>
                {
                    let handle = objects.$store.reserve();
                    let object = (handle.clone(), self).into();
                    objects.insert(object);
                    handle
                }
            }
        )*
    };
}

impl_insert!(
    Cycle, cycles;
    Face, faces;
    GlobalEdge, global_edges;
    HalfEdge, half_edges;
    Shell, shells;
    Sketch, sketches;
    Solid, solids;
    Surface, surfaces;
    Vertex, vertices;
);
