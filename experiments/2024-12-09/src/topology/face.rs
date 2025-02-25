use std::fmt;

use itertools::Itertools;

use crate::{
    extra::triangulate::triangulate,
    geometry::TriMesh,
    math::Plane,
    object::{Handle, HandleAny, Object},
};

use super::{half_edge::HalfEdge, vertex::Vertex};

#[derive(Debug)]
pub struct Face {
    pub surface: Plane,
    pub half_edges: Vec<Handle<HalfEdge>>,
    pub is_internal: bool,
}

impl Face {
    pub fn new(
        surface: Plane,
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
