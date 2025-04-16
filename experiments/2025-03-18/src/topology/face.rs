use fj_interop::{Tolerance, TriMesh};
use itertools::Itertools;

use crate::{
    extra::triangulate::triangulate, geometry::ToTriMesh, handle::Handle,
};

use super::{half_edge::HalfEdge, surface::Surface, vertex::Vertex};

#[derive(Debug)]
pub struct Face {
    pub surface: Handle<Surface>,
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
    ) -> impl Iterator<Item = HalfEdgeWithEndVertex> {
        self.half_edges
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| HalfEdgeWithEndVertex {
                half_edge: a,
                end_vertex: &b.start,
            })
    }
}

impl ToTriMesh for Face {
    fn to_tri_mesh(&self, tolerance: impl Into<Tolerance>) -> TriMesh {
        triangulate(self, tolerance)
    }
}

pub struct HalfEdgeWithEndVertex<'r> {
    pub half_edge: &'r Handle<HalfEdge>,
    pub end_vertex: &'r Handle<Vertex>,
}
