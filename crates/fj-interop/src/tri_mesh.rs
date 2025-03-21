use std::collections::HashMap;

use fj_math::{Aabb, Point};

use crate::Color;

/// A triangle mesh
#[derive(Clone, Debug, Default)]
pub struct TriMesh {
    vertices: Vec<Point<3>>,
    indices: Vec<Index>,

    indices_by_vertex: HashMap<Point<3>, Index>,
    triangles: Vec<Triangle>,
}

impl TriMesh {
    /// Construct a new instance of `Mesh`
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a triangle to the mesh
    pub fn push_triangle(
        &mut self,
        triangle: impl Into<fj_math::Triangle<3>>,
        color: Color,
    ) {
        let triangle = triangle.into();

        for point in triangle.points {
            let index =
                *self.indices_by_vertex.entry(point).or_insert_with(|| {
                    let index = self.vertices.len();
                    self.vertices.push(point);
                    index as u32
                });

            self.indices.push(index);
        }

        self.triangles.push(Triangle {
            inner: triangle,
            color,
        });
    }

    /// Determine whether the mesh contains the provided triangle
    ///
    /// Returns true, if a triangle with any combination of the provided points
    /// is part of the mesh.
    pub fn contains_triangle(
        &self,
        triangle: impl Into<fj_math::Triangle<3>>,
    ) -> bool {
        let triangle = triangle.into().normalize();

        for t in &self.triangles {
            let t = t.inner.normalize();
            if triangle == t {
                return true;
            }
        }

        false
    }

    /// Access the vertices of the mesh
    pub fn vertices(&self) -> impl Iterator<Item = Point<3>> + '_ {
        self.vertices.iter().copied()
    }

    /// Access the indices of the mesh
    pub fn indices(&self) -> impl Iterator<Item = Index> + '_ {
        self.indices.iter().copied()
    }

    /// Access the triangles of the mesh
    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.triangles.iter().copied()
    }

    /// # Compute the axis-aligned bounding box of this mesh
    pub fn aabb(&self) -> Aabb<3> {
        Aabb::<3>::from_points(self.vertices.iter().copied())
    }
}

/// An index that refers to a vertex in a mesh
pub type Index = u32;

/// A triangle
///
/// Extension of [`fj_math::Triangle`] that also includes a color.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Triangle {
    /// The points of the triangle
    pub inner: fj_math::Triangle<3>,

    /// The color of the triangle
    pub color: Color,
}
