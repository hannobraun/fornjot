// TASK: Split `Mesh` and `MeshBuilder`. `MeshBuilder` (`MeshMaker`)? would be
//       an API for creating meshes. It would would inherit most of the
//       complexity of the current `Mesh`. `Mesh` would become much simpler,
//       something like `threemf::TriangleMesh` or `graphics::Vertices`.

use std::collections::HashMap;

use crate::{math::Point, types::Index, util};

use super::Triangle;

/// A triangle mesh
pub struct Mesh<const D: usize> {
    vertices: Vec<Point<D>>,
    triangles: HashMap<Triangle<D>, [Index; 3]>,
}

impl<const D: usize> Mesh<D> {
    /// Iterate over all vertices
    pub fn vertices(&self) -> impl Iterator<Item = Point<D>> + '_ {
        self.vertices.iter().copied()
    }

    /// Iterate over all indices
    pub fn indices(&self) -> impl Iterator<Item = Index> + '_ {
        self.triangles.values().flatten().copied()
    }

    /// Iterate over the vertices that make up all triangles
    pub fn triangle_vertices(&self) -> impl Iterator<Item = Triangle<D>> + '_ {
        self.triangles.keys().copied()
    }

    /// Iterate over the indices that make up all triangles
    pub fn triangle_indices(&self) -> impl Iterator<Item = [Index; 3]> + '_ {
        self.triangles.values().copied()
    }

    /// Map all vertices
    ///
    /// This method is intended for testing only. It is going to corrupt the
    /// `Mesh`'s internal state, only leaving some methods functional.
    pub fn map(&mut self, mut f: impl FnMut(Point<D>) -> Point<D>) {
        self.triangles = self
            .triangles
            .clone()
            .into_iter()
            .map(|(triangle, indices)| {
                let points = triangle.points().map(&mut f);
                (Triangle::from_points(points).unwrap(), indices)
            })
            .collect()
    }
}

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
        Mesh {
            vertices: self.vertices.iter().collect(),
            triangles: self.triangles.clone(),
        }
    }
}
