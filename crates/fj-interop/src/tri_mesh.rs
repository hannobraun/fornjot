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

    /// # Add all the triangles from another `TriMesh` to this one
    ///
    /// This is literally all this method does. There's nothing fancy going on,
    /// no de-duplication, no validation. Make sure this is what you want!
    pub fn merge(mut self, other: Self) -> Self {
        self.triangles.extend(other.triangles);
        self
    }

    /// # Iterate over all triangles in this mesh
    pub fn all_triangles(&self) -> impl Iterator<Item = Triangle<3>> {
        self.triangles.iter().map(|triangle| triangle.inner)
    }

    /// # Iterate over only the external triangles in this mesh
    ///
    /// See documentation of [`MeshTriangle`] for an explanation of internal and
    /// external triangles.
    pub fn external_triangles(&self) -> impl Iterator<Item = Triangle<3>> {
        self.triangles.iter().filter_map(|triangle| {
            (!triangle.is_internal).then_some(triangle.inner)
        })
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

    /// # Compute the axis-aligned bounding box of this mesh
    pub fn aabb(&self) -> Aabb<3> {
        Aabb::<3>::from_points(
            self.triangles
                .iter()
                .flat_map(|triangle| triangle.inner.points),
        )
    }
}

/// # A triangle in a [`TriMesh`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MeshTriangle {
    /// # The triangle
    pub inner: Triangle<3>,

    /// # Track whether this is an internal triangle
    ///
    /// Internal triangles are triangles that are inside of the solid body that
    /// the triangle mesh is a boundary for.
    ///
    /// If the triangle mesh is well-formed, then internal triangles are still
    /// part of a single contiguous outer shell. But this shell might touch
    /// itself, making the triangles where that is the case internal.
    pub is_internal: bool,

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
