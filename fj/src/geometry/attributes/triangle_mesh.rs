use crate::{math::Point, types::Index, util};

/// Implemented for geometry that can be converted to a triangle mesh
pub trait TriangleMesh<const D: usize> {
    /// Convert geometry to a triangle mesh
    fn mesh(&self) -> Mesh<D>;
}

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
    pub fn triangle(
        &mut self,
        v0: impl Into<Point<D>>,
        v1: impl Into<Point<D>>,
        v2: impl Into<Point<D>>,
    ) {
        let v0 = v0.into();
        let v1 = v1.into();
        let v2 = v2.into();

        // Make sure this is a real triangle.
        assert_ne!(v0, v1);
        assert_ne!(v0, v2);
        assert_ne!(v1, v2);

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

    /// Iterate over all triangles
    pub fn triangles(&self) -> impl Iterator<Item = [Index; 3]> + '_ {
        self.triangles.iter().copied()
    }
}
