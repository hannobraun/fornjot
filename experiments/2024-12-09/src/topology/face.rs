use std::fmt;

use itertools::Itertools;

use crate::{
    extra::triangulate::triangulate,
    geometry::TriMesh,
    object::{Handle, HandleAny, Object},
};

use super::{half_edge::HalfEdge, surface::Surface, vertex::Vertex};

/// # A face
///
/// Faces are defined by a surface (which, so far, is always a plane) and a
/// cycle of half-edges that bound the face on that surface.
///
/// Faces are the boundary of any solid. Solids can touch themselves, however,
/// to connect their external boundary to cavities on the inside, or enclose a
/// hole through the solid.
///
/// The faces in parts of the boundary where solids touch themselves are called
/// "internal".
#[derive(Debug)]
pub struct Face {
    pub surface: Handle<Surface>,
    pub half_edges: Vec<Handle<HalfEdge>>,
    pub is_internal: bool,
}

impl Face {
    /// # Create a new face from its component parts
    ///
    /// The more interesting way to create a face would be via a
    /// [`Sketch`](crate::geometry::Sketch).
    pub fn new(
        surface: Handle<Surface>,
        half_edges: impl IntoIterator<Item = Handle<HalfEdge>>,
        is_internal: bool,
    ) -> Self {
        Self {
            surface,
            half_edges: half_edges.into_iter().collect(),
            is_internal,
        }
    }

    /// # Iterate over the half-edges of the face
    ///
    /// In addition to the [`HalfEdge`] itself, which contains the vertex where
    /// it starts, the vertex where the half-edge ends (the start vertex of the
    /// next half-edge) is also provided.
    pub fn half_edges_with_end_vertex(
        &self,
    ) -> impl Iterator<Item = (&Handle<HalfEdge>, &Handle<Vertex>)> {
        self.half_edges
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| (a, &b.start))
    }
}

impl Object for Face {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Face")
    }

    fn tri_mesh(&self) -> TriMesh {
        triangulate(self)
    }

    fn children(&self) -> Vec<HandleAny> {
        self.half_edges
            .iter()
            .map(|vertex| vertex.to_any())
            .collect()
    }
}
