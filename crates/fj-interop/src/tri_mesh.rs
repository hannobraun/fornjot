use std::collections::{BTreeMap, HashMap};

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

/// # Convert a sequence of vertices into unique vertices and indices
///
/// Takes an iterator over vertices and converts those into two values:
///
/// - A collection of vertices that contains all the original vertices, but each
///   only once, with the order being undefined.
/// - A collection of indices that correspond one-to-one to the original
///   vertices reflect their original order.
///
/// The indices map into the collection of vertices.
///
/// Some environments, like renderers or some file formats, expect this kind of
/// setup. These environments tend to also expect custom vertex formats, and
/// those vertex formats might not fulfill the requirements needed to do the
/// kind of comparing and sorting this function does.
///
/// For this reason, a closure that converts the original vertices into another
/// format, after the comparing and sorting is done, is also accepted. This
/// might save the caller from having to allocate another collection.
pub fn vertices_to_indexed_vertices<I, O>(
    original_vertices: impl IntoIterator<Item = I>,
    map_vertex: impl Fn(I) -> O,
) -> (Vec<O>, Vec<Index>)
where
    I: Copy + Ord,
{
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let mut indices_by_vertex = BTreeMap::new();

    for vertex in original_vertices.into_iter() {
        let index = *indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(map_vertex(vertex));
            index as u32
        });

        indices.push(index);
    }

    (vertices, indices)
}

/// An index that refers to a vertex in a mesh
pub type Index = u32;
