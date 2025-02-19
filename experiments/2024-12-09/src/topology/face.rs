use std::fmt;

use itertools::Itertools;

use crate::{
    extra::triangulate::triangulate,
    geometry::TriMesh,
    math::Plane,
    object::{Handle, HandleAny, Object},
};

use super::vertex::Vertex;

#[derive(Debug)]
pub struct Face {
    surface: Plane,
    vertices: Vec<Handle<Vertex>>,
}

impl Face {
    pub fn new(
        surface: Plane,
        vertices: impl IntoIterator<Item = Handle<Vertex>>,
    ) -> Self {
        Self {
            surface,
            vertices: vertices.into_iter().collect(),
        }
    }

    pub fn surface(&self) -> &Plane {
        &self.surface
    }

    pub fn vertices(&self) -> impl Iterator<Item = &Handle<Vertex>> {
        self.vertices.iter()
    }

    pub fn half_edges(&self) -> impl Iterator<Item = [&Handle<Vertex>; 2]> {
        self.vertices
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| [a, b])
    }
}

impl Object for Face {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Face")
    }

    fn tri_mesh(&self) -> TriMesh {
        triangulate(self.vertices(), self.surface())
    }

    fn children(&self) -> Vec<HandleAny> {
        self.vertices.iter().map(|vertex| vertex.to_any()).collect()
    }
}
