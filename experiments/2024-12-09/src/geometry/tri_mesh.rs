use super::Triangle;

/// # A triangle mesh
///
/// Triangle meshes are the uniform intermediate representation for geometry.
/// The idea here is to have a single representation that is both (relatively)
/// easy to generate and to operate on.
///
/// This is only intended as an _intermediate_ representation though! This isn't
/// fully worked out yet in this experiment, but the idea is to keep the
/// original objects that generated the triangle mesh around, so you can always
/// generate a more accurate triangle mesh, if needed.
#[derive(Debug)]
pub struct TriMesh {
    pub triangles: Vec<MeshTriangle>,
}

impl TriMesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    /// # Merge this triangle mesh with another
    ///
    /// This just creates a new triangle mesh that has all the triangles from
    /// both meshes. Nothing more fancy than that, so far!
    pub fn merge(mut self, other: Self) -> Self {
        self.triangles.extend(other.triangles);
        self
    }

    /// # Iterate over all the triangles in the mesh
    pub fn all_triangles(&self) -> impl Iterator<Item = Triangle<3>> {
        self.triangles.iter().map(|triangle| triangle.inner)
    }

    /// # Iterate over the triangles in the mesh that are not marked internal
    ///
    /// See [`MeshTriangle`] for an explanation of internal and external
    /// triangles.
    pub fn external_triangles(&self) -> impl Iterator<Item = Triangle<3>> {
        self.triangles.iter().filter_map(|triangle| {
            (!triangle.is_internal).then_some(triangle.inner)
        })
    }
}

/// # A triangle in a triangle mesh
///
/// This is just a regular triangle, with an additional flag to mark it as
/// internal.
///
/// Faces only ever have a single boundary. Holes are realized by having this
/// boundary touch itself in one place, where it connects the inside and the
/// outside.
///
/// The half-edges where that happens are marked as "internal", and so are any
/// triangles created from them. This method can be used to filter those out,
/// for example for export to external file formats.
#[derive(Debug)]
pub struct MeshTriangle {
    pub inner: Triangle<3>,
    pub is_internal: bool,
}
