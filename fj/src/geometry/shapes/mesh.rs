use std::collections::HashMap;

use crate::{math::Point, types::Index, util};

use super::{Quad, Triangle};

/// A triangle mesh
pub struct Mesh<const D: usize> {
    vertices: util::Vertices<Point<D>, D>,
    triangles: HashMap<Triangle<D>, [Index; 3]>,
}

impl<const D: usize> Mesh<D> {
    /// Create an empty triangle mesh
    pub fn new() -> Self {
        Self {
            vertices: util::Vertices::new(),
            triangles: HashMap::new(),
        }
    }

    /// Add a triangle to the mesh
    ///
    /// # Panics
    ///
    /// Panics, if the three vertices don't form a triangle (i.e. if at least
    /// two of them are equal).
    pub fn triangle(&mut self, triangle: Triangle<D>) {
        let [v0, v1, v2] = triangle.points();

        let i0 = self.vertices.index_for_vertex(v0);
        let i1 = self.vertices.index_for_vertex(v1);
        let i2 = self.vertices.index_for_vertex(v2);

        self.triangles.insert(triangle, [i0, i1, i2]);
    }

    /// Iterate over all vertices
    pub fn vertices(&self) -> impl Iterator<Item = Point<D>> + '_ {
        self.vertices.iter()
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

    /// Indicate, whether the mesh contains triangles that make up a quad
    pub fn contains_quad(&self, quad: &Quad<D>) -> bool {
        let [a, b, c, d] = quad.points();

        // Neither of the following triangle constructions can panic, as the
        // points come from a quad, meaning they are already validated.

        let abc = Triangle::from_points([a, b, c]).unwrap();
        let acd = Triangle::from_points([a, c, d]).unwrap();

        let abd = Triangle::from_points([a, b, d]).unwrap();
        let bcd = Triangle::from_points([b, c, d]).unwrap();

        self.triangles.contains_key(&abc) && self.triangles.contains_key(&acd)
            || self.triangles.contains_key(&abd)
                && self.triangles.contains_key(&bcd)
    }

    /// Round all coordinates of all triangles
    ///
    /// This method is intended for testing only. It is going to corrupt the
    /// `Mesh`'s internal state, only leaving some methods functional.
    pub fn round(&mut self) {
        fn round(v: f32) -> f32 {
            (v * 100.).round() / 100.
        }

        self.triangles = self
            .triangles
            .clone()
            .into_iter()
            .map(|(triangle, indices)| {
                let mut points = triangle.points();

                for point in &mut points {
                    *point = point.map(|coord| round(coord));
                }

                (Triangle::from_points(points).unwrap(), indices)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::{Quad, Triangle};

    use super::Mesh;

    #[test]
    fn test_contains_quad() {
        let a = [0., 0.];
        let b = [1., 0.];
        let c = [1., 1.];
        let d = [0., 1.];

        // Quad 1
        let abc = Triangle::from_points([a, b, c]).unwrap();
        let acd = Triangle::from_points([a, c, d]).unwrap();

        // Quad 2
        let abd = Triangle::from_points([a, b, d]).unwrap();
        let bcd = Triangle::from_points([b, c, d]).unwrap();

        let quad = Quad::from_points([a, b, c, d]).unwrap();

        let mut mesh = Mesh::new();
        assert!(!mesh.contains_quad(&quad));
        mesh.triangle(abc);
        assert!(!mesh.contains_quad(&quad));
        mesh.triangle(acd);
        assert!(mesh.contains_quad(&quad));

        let mut mesh = Mesh::new();
        assert!(!mesh.contains_quad(&quad));
        mesh.triangle(abd);
        assert!(!mesh.contains_quad(&quad));
        mesh.triangle(bcd);
        assert!(mesh.contains_quad(&quad));
    }
}
