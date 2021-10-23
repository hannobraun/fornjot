/// A triangle mesh
///
/// This is a very basic types that lacks any amenities for constructing it or
/// for iterating over its data.
///
/// This is by design. Providing a generally usable and feature-rich triangle
/// mesh type is out of scope for this library. It is expected that users of
/// this library will use their own mesh type anyway, and the simplicity of
/// `TriangleMesh` provides an easy target for conversion from such a type.
pub struct TriangleMesh {
    /// The vertices of the mesh
    ///
    /// This defines the vertices that are part of the mesh, but not the mesh's
    /// structure. See the `triangles` field.
    pub vertices: Vec<Vertex>,

    /// The triangles that make up the mesh
    ///
    /// Each triangle consists of indices that refer back to the `vertices`
    /// field.
    pub triangles: Vec<IndexTriangle>,
}

/// A vertex in a triangle mesh
///
/// See [`TriangleMesh`].
pub type Vertex = [f32; 3];

/// A triangle in a triangle mesh
///
/// The triangle consists of indices that refer to the vertices of the mesh. See
/// [`TriangleMesh`].
pub type IndexTriangle = [Index; 3];

/// An index that refers to a vertex in a triangle mesh
///
/// See [`TriangleMesh`] and [`IndexTriangle`].
pub type Index = usize;
