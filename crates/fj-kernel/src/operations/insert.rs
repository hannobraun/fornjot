use crate::{
    objects::{
        Cycle, Face, GlobalEdge, HalfEdge, Shell, Sketch, Solid, Surface,
        Vertex,
    },
    services::Services,
    storage::Handle,
};

use super::Polygon;

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

/// Indicate whether an object has been inserted
///
/// Intended to be used as a type parameter bound for structs that need to track
/// whether their contents have been inserted or not.
pub trait IsInserted {
    /// The type of the object for which the insertion status is tracked
    type T<T>;
}

/// Indicate that an object has been inserted
///
/// See [`IsInserted`].
pub struct IsInsertedYes;

impl IsInserted for IsInsertedYes {
    type T<T> = Handle<T>;
}

/// Indicate that an object has not been inserted
///
/// See [`IsInserted`].
pub struct IsInsertedNo;

impl IsInserted for IsInsertedNo {
    type T<T> = T;
}

impl<const D: usize> Insert for Polygon<D, IsInsertedNo> {
    type Inserted = Polygon<D, IsInsertedYes>;

    fn insert(self, services: &mut Services) -> Self::Inserted {
        Polygon {
            face: self.face.insert(services),
            edges: self.edges,
            vertices: self.vertices,
        }
    }
}
