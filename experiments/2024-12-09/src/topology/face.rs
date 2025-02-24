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
    surface: Plane,
    half_edges: Vec<Handle<HalfEdge>>,
}

impl Face {
    pub fn new(
        surface: Plane,
        half_edges: impl IntoIterator<Item = Handle<HalfEdge>>,
    ) -> Self {
        Self {
            surface,
            half_edges: half_edges.into_iter().collect(),
        }
    }

    pub fn surface(&self) -> &Plane {
        &self.surface
    }

    pub fn half_edges(&self) -> impl Iterator<Item = &Handle<HalfEdge>> {
        self.half_edges.iter()
    }

    pub fn start_and_end_vertices(
        &self,
    ) -> impl Iterator<Item = [&Handle<Vertex>; 2]> {
        self.half_edges
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| [a.start(), b.start()])
    }
}

impl Object for Face {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Face")
    }

    fn tri_mesh(&self) -> TriMesh {
        triangulate(&self.half_edges, self.surface())
    }

    fn children(&self) -> Vec<HandleAny> {
        self.half_edges().map(|vertex| vertex.to_any()).collect()
    }
}
