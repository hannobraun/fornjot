use crate::{
    operations::build::{Polygon, TetrahedronShell},
    storage::Handle,
    topology::{
        Curve, Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid, Surface,
        Vertex,
    },
    Core,
};

use super::{IsInsertedNo, IsInsertedYes};

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
    ///
    /// Inserted objects will automatically be validated in the background. You
    /// should not insert an invalid object into the stores, unless you have a
    /// specific reason to do so, and you are handling validation errors in a
    /// non-standard way.
    #[must_use]
    fn insert(self, core: &mut Core) -> Self::Inserted;
}

macro_rules! impl_insert {
    ($($ty:ty, $store:ident;)*) => {
        $(
            impl Insert for $ty {
                type Inserted = Handle<Self>;

                fn insert(self, core: &mut Core) -> Self::Inserted {
                    let handle = core.layers.topology.$store.reserve();
                    let object = (handle.clone(), self).into();
                    core.layers.topology.insert(
                        object,
                        &mut core.layers.geometry,
                        &mut core.layers.validation,
                    );
                    handle
                }
            }
        )*
    };
}

impl_insert!(
    Curve, curves;
    Cycle, cycles;
    Face, faces;
    HalfEdge, half_edges;
    Region, regions;
    Shell, shells;
    Sketch, sketches;
    Solid, solids;
    Surface, surfaces;
    Vertex, vertices;
);

// Implement `Insert` for `Handle` as a no-op. This is useful for code that
// needs a `Handle` in the end, but doesn't care if it gets that directly or
// inserts a bare object itself.
impl<T> Insert for Handle<T>
where
    T: Insert,
{
    type Inserted = Self;

    fn insert(self, _: &mut Core) -> Self::Inserted {
        self
    }
}

impl<const D: usize> Insert for Polygon<D, IsInsertedNo> {
    type Inserted = Polygon<D, IsInsertedYes>;

    fn insert(self, core: &mut Core) -> Self::Inserted {
        Polygon {
            face: self.face.insert(core),
            half_edges: self.half_edges,
            vertices: self.vertices,
        }
    }
}

impl Insert for TetrahedronShell<IsInsertedNo> {
    type Inserted = TetrahedronShell<IsInsertedYes>;

    fn insert(self, core: &mut Core) -> Self::Inserted {
        TetrahedronShell {
            shell: self.shell.insert(core),
            abc: self.abc,
            bad: self.bad,
            dac: self.dac,
            cbd: self.cbd,
        }
    }
}
