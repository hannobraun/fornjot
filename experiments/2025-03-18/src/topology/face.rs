use fj_interop::{Tolerance, TriMesh};
use itertools::Itertools;

use crate::{extra::triangulate::triangulate_face, handle::Handle};

use super::{half_edge::HalfEdge, surface::Surface, vertex::Vertex};

#[derive(Debug)]
pub struct Face {
    pub surface: Handle<Surface>,

    /// # The half-edges that bound the face on the surface
    ///
    /// Half-edges are specific to one face. They are never shared with another
    /// one. Hence, we don't need a [`Handle`] here.We could instead own the
    /// `HalfEdge`s directly.
    ///
    /// This is probably a good idea, as it would simplify the object graph.
    /// Though there's also another consideration to make here: while half-edges
    /// are unique to a face, there is a (so far) implicit concept of "edges"
    /// which are shared between faces.
    ///
    /// It might be advantageous to make this explicit, by having an `Edge`
    /// struct that is referred to as `Handle<Edge>`. Not only would this better
    /// communicate the intent of the design, it would also allow for easier
    /// caching of approximations, or inferring if a half-edge is internal.
    ///
    /// `Edge` could refer to its two bounding vertices (as `Handle<Vertex>`),
    /// and the curve (as `Handle<Curve>`). Then `HalfEdge` would be reduced to
    /// a `Handle<Edge>`, a field to specify the direction of the half-edge, and
    /// possibly its current `is_internal` field.
    pub half_edges: Vec<Handle<HalfEdge>>,

    pub is_internal: bool,
}

impl Face {
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

    pub fn half_edges_with_end_vertex(
        &self,
    ) -> impl Iterator<Item = HalfEdgeWithEndVertex<'_>> {
        self.half_edges
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| HalfEdgeWithEndVertex {
                half_edge: a,
                end_vertex: &b.start,
            })
    }

    pub fn to_tri_mesh(&self, tolerance: impl Into<Tolerance>) -> TriMesh {
        triangulate_face(self, tolerance)
    }
}

pub struct HalfEdgeWithEndVertex<'r> {
    pub half_edge: &'r Handle<HalfEdge>,
    pub end_vertex: &'r Handle<Vertex>,
}
