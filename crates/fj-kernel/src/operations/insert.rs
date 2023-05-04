use crate::{
    objects::{
        Cycle, Face, GlobalEdge, HalfEdge, Shell, Sketch, Solid, Surface,
        Vertex,
    },
    services::Services,
    storage::Handle,
};

/// Insert an object into its respective store
///
/// This is the only primitive operation that is directly understood by
/// `Service<Objects>`. All other operations are built on top of it.
pub trait Insert: Sized {
    /// The type of `Self`, once it has been inserted
    ///
    /// Usually this is just `Handle<Self>`, but there are some more complex
    /// cases where this type needs to be customized.
    type Inserted;

    /// Insert the object into its respective store
    fn insert(self, services: &mut Services) -> Self::Inserted;
}

macro_rules! impl_insert {
    ($($ty:ty, $store:ident;)*) => {
        $(
            impl Insert for $ty {
                type Inserted = Handle<Self>;

                fn insert(self, services: &mut Services) -> Self::Inserted {
                    let handle = services.objects.$store.reserve();
                    let object = (handle.clone(), self).into();
                    services.insert_object(object);
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
