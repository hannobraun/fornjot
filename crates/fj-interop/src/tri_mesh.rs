use std::collections::BTreeMap;

use fj_math::{Aabb, Triangle};

use crate::Color;

/// # A triangle mesh
#[derive(Clone, Debug, Default)]
pub struct TriMesh {
    /// # The triangles in the mesh
    pub triangles: Vec<MeshTriangle>,
}

impl TriMesh {
    /// Construct a new instance of `Mesh`
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a triangle to the mesh
    pub fn push_triangle(
        &mut self,
        triangle: impl Into<Triangle<3>>,
        color: Color,
    ) {
        self.triangles.push(MeshTriangle {
            inner: triangle.into(),
            color,
        });
    }

    /// Determine whether the mesh contains the provided triangle
    ///
    /// Returns true, if a triangle with any combination of the provided points
    /// is part of the mesh.
    pub fn contains_triangle(&self, triangle: impl Into<Triangle<3>>) -> bool {
        let triangle = triangle.into().normalize();

        for t in &self.triangles {
            let t = t.inner.normalize();
            if triangle == t {
                return true;
            }
        }

        false
    }

    /// Access the triangles of the mesh
    pub fn triangles(&self) -> impl Iterator<Item = MeshTriangle> + '_ {
        self.triangles.iter().copied()
    }

    /// # Compute the axis-aligned bounding box of this mesh
    pub fn aabb(&self) -> Aabb<3> {
        Aabb::<3>::from_points(
            self.triangles().flat_map(|triangle| triangle.inner.points),
        )
    }
}

/// # A triangle in a [`TriMesh`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MeshTriangle {
    /// # The triangle
    pub inner: Triangle<3>,

    /// # The color of the triangle
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
