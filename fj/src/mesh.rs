use crate::math::Point;

/// A triangle mesh
#[derive(Debug)]
pub struct Mesh<const D: usize> {
    vertices: Vec<Point<D>>,
    triangles: Vec<[Index; 3]>,
}

impl<const D: usize> Mesh<D> {
    /// Create a new instance of `Mesh`
    ///
    /// This method expects the vertices that make up the mesh, as well as the
    /// triangles that provide the structure. The triangles consist of indices
    /// that index into `vertices`.
    ///
    /// At this point in time, no validation is done to verify that the indices
    /// are valid.
    pub fn new(vertices: Vec<Point<D>>, triangles: Vec<[Index; 3]>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }

    /// Iterate over all vertices
    pub fn vertices(&self) -> impl Iterator<Item = Point<D>> + '_ {
        self.vertices.iter().copied()
    }

    /// Iterate over all indices
    pub fn indices(&self) -> impl Iterator<Item = Index> + '_ {
        self.triangles.iter().flatten().copied()
    }

    /// Iterate over the vertices that make up all triangles
    pub fn triangle_vertices(
        &self,
    ) -> impl Iterator<Item = [Point<D>; 3]> + '_ {
        self.triangles.iter().copied().map(|[i1, i2, i3]| {
            let v1 = self.vertices[i1 as usize];
            let v2 = self.vertices[i2 as usize];
            let v3 = self.vertices[i3 as usize];

            [v1, v2, v3]
        })
    }

    /// Iterate over the indices that make up all triangles
    pub fn triangle_indices(&self) -> impl Iterator<Item = [Index; 3]> + '_ {
        self.triangles.iter().copied()
    }

    /// Map all vertices
    ///
    /// This method is intended for testing only. It is going to corrupt the
    /// `Mesh`'s internal state, only leaving some methods functional.
    pub fn map(&mut self, f: impl FnMut(Point<D>) -> Point<D>) {
        self.vertices = self.vertices.iter().copied().map(f).collect()
    }
}

pub type Index = u32;
