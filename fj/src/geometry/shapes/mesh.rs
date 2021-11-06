use std::collections::HashMap;

use crate::{math::Point, mesh::Index, util};

use super::Triangle;

pub use crate::mesh::Mesh;

/// API for creating `Mesh`es
pub struct MeshMaker<const D: usize> {
    vertices: util::Vertices<Point<D>, D>,
    triangles: HashMap<Triangle<D>, [Index; 3]>,
}

impl<const D: usize> MeshMaker<D> {
    /// Create a new `MeshMaker`
    pub fn new() -> Self {
        Self {
            vertices: util::Vertices::new(),
            triangles: HashMap::new(),
        }
    }

    /// Add a triangle to the mesh
    pub fn triangle(&mut self, triangle: Triangle<D>) {
        let [v0, v1, v2] = triangle.points();

        let i0 = self.vertices.index_for_vertex(v0);
        let i1 = self.vertices.index_for_vertex(v1);
        let i2 = self.vertices.index_for_vertex(v2);

        self.triangles.insert(triangle, [i0, i1, i2]);
    }

    pub fn make(&self) -> Mesh<D> {
        let vertices = self.vertices.iter().collect();
        let triangles = self.triangles.values().copied().collect();

        Mesh::new(vertices, triangles)
    }
}
