//! A triangle mesh

use std::{collections::HashMap, hash::Hash};

use fj_math::Point;

/// A triangle mesh
pub struct Mesh<V> {
    vertices: Vec<V>,
    indices: Vec<Index>,

    indices_by_vertex: HashMap<V, Index>,
    triangles: Vec<Triangle>,
}

impl<V> Mesh<V>
where
    V: Copy + Eq + Hash,
{
    /// Construct a new instance of `Mesh`
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a vertex to the mesh
    pub fn push_vertex(&mut self, vertex: V) {
        let index =
            *self.indices_by_vertex.entry(vertex).or_insert_with(|| {
                let index = self.vertices.len();
                self.vertices.push(vertex);
                index as u32
            });

        self.indices.push(index);
    }

    /// Determine whether the mesh contains the provided triangle
    ///
    /// Returns true, if a triangle with any combination of the points of the
    /// provided triangle is part of the mesh.
    pub fn contains_triangle(
        &self,
        triangle: [impl Into<Point<3>>; 3],
    ) -> bool {
        let triangle = fj_math::Triangle::from_points(triangle).normalize();

        for t in &self.triangles {
            let t = fj_math::Triangle::from_points(t.points).normalize();
            if triangle == t {
                return true;
            }
        }

        false
    }

    /// Access the vertices of the mesh
    pub fn vertices(&self) -> impl Iterator<Item = V> + '_ {
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
}

impl Mesh<Point<3>> {
    /// Add a triangle to the mesh
    pub fn push_triangle(&mut self, points: [Point<3>; 3], color: Color) {
        for point in points {
            self.push_vertex(point);
        }

        self.triangles.push(Triangle { points, color });
    }
}

// This needs to be a manual implementation. Deriving `Default` would require
// `V` to be `Default` as well, even though that is not necessary.
impl<V> Default for Mesh<V> {
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            indices: Default::default(),
            indices_by_vertex: Default::default(),
            triangles: Default::default(),
        }
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
    pub points: [Point<3>; 3],

    /// The color of the triangle
    pub color: Color,
}

/// RGBA color
pub type Color = [u8; 4];
