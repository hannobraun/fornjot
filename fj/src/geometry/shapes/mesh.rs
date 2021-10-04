use crate::{math::Point, types::Index, util};

use super::Triangle;

/// A triangle mesh
pub struct Mesh<const D: usize> {
    vertices: util::Vertices<Point<D>, D>,
    triangles: Vec<[Index; 3]>,
}

impl<const D: usize> Mesh<D> {
    /// Create an empty triangle mesh
    pub fn new() -> Self {
        Self {
            vertices: util::Vertices::new(),
            triangles: Vec::new(),
        }
    }

    /// Add a triangle to the mesh
    ///
    /// # Panics
    ///
    /// Panics, if the three vertices don't form a triangle (i.e. if at least
    /// two of them are equal).
    pub fn triangle(&mut self, triangle: Triangle<D, 3>) {
        let [v0, v1, v2] = triangle.points();

        let i0 = self.vertices.index_for_vertex(v0);
        let i1 = self.vertices.index_for_vertex(v1);
        let i2 = self.vertices.index_for_vertex(v2);

        self.triangles.push([i0, i1, i2]);
    }

    /// Iterate over all vertices
    pub fn vertices(&self) -> impl Iterator<Item = Point<D>> + '_ {
        self.vertices.iter()
    }

    /// Iterate over all indices
    pub fn indices(&self) -> impl Iterator<Item = Index> + '_ {
        self.triangles.iter().flatten().copied()
    }

    /// Iterate over the vertices that make up all triangles
    pub fn triangle_vertices(
        &self,
    ) -> impl Iterator<Item = Triangle<D, 3>> + '_ {
        self.triangles.iter().copied().map(move |[a, b, c]| {
            Triangle::new(
                self.vertices.vertex(a),
                self.vertices.vertex(b),
                self.vertices.vertex(c),
            )
            // This should never panic, as the vertices were originally taken
            // from a `Triangle`, thus should not fail to form a new `Triangle`.
            .expect("Failed to construct `Triangle`")
        })
    }

    /// Iterate over the indices that make up all triangles
    pub fn triangle_indices(&self) -> impl Iterator<Item = [Index; 3]> + '_ {
        self.triangles.iter().copied()
    }
}
