use std::collections::HashMap;

use decorum::R32;

use crate::math::Point;

/// A triangle mesh
#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<IndexTriangle>,
}

impl Mesh {
    /// The vertices that make up the triangle mesh
    pub fn vertices(&self) -> impl Iterator<Item = Vertex> + '_ {
        self.vertices.iter().copied()
    }

    /// The triangles that provide the structure of the triangle mesh
    ///
    /// Each triangle consists of indices that index into `vertices`.
    pub fn triangles(&self) -> impl Iterator<Item = IndexTriangle> + '_ {
        self.triangles.iter().copied()
    }
}

/// A vertex in a triangle mesh
///
/// See [`Mesh`].
pub type Vertex = Point;

/// An index that refers to a vertex
///
/// See [`Mesh`].
///
/// Since this type is used to index into a [`Vec`], [`usize`] would seem to be the
/// natural type to use here. We're using this for computer graphics, however,
/// and for that we need a type with defined size.
pub type Index = u32;

/// A triangle consisting of indices that refer to mesh vertices
///
/// See [`Mesh`].
pub type IndexTriangle = [Index; 3];

/// A triangle consisting of mesh vertices
///
/// See [`Mesh`].
pub type VertexTriangle = [Vertex; 3];

/// API for creating [`Mesh`]es
pub struct MeshMaker {
    vertices: Vertices,
    triangles: Vec<IndexTriangle>,
}

impl MeshMaker {
    /// Create a new instance of `MeshMaker`
    pub fn new() -> Self {
        Self {
            vertices: Vertices::new(),
            triangles: Vec::new(),
        }
    }

    /// Add a triangle to the mesh
    pub fn triangle(&mut self, triangle: VertexTriangle) {
        let [v0, v1, v2] = triangle;

        let i0 = self.vertices.index_for_vertex(v0);
        let i1 = self.vertices.index_for_vertex(v1);
        let i2 = self.vertices.index_for_vertex(v2);

        self.triangles.push([i0, i1, i2]);
    }

    pub fn make(self) -> Mesh {
        Mesh {
            vertices: self.vertices.into_vec(),
            triangles: self.triangles,
        }
    }
}

/// Utility struct for managing triangle mesh vertices
pub struct Vertices {
    vertices: Vec<Vertex>,
    indices_by_vertex: HashMap<HashVertex, Index>,
}

impl Vertices {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices_by_vertex: HashMap::new(),
        }
    }

    pub fn index_for_vertex(&mut self, vertex: Vertex) -> Index {
        let hash_vertex = vertex.coords.data.0[0].map(|coord| coord.into());

        let index =
            self.indices_by_vertex
                .entry(hash_vertex)
                .or_insert_with(|| {
                    let index = self.vertices.len();
                    self.vertices.push(vertex);
                    index.try_into().expect("Couldn't convert `usize` to `u32`")
                });

        *index
    }

    pub fn into_vec(self) -> Vec<Vertex> {
        self.vertices
    }
}

/// A vertex that can be used as the key of a [`HashMap`]
pub type HashVertex = [R32; 3];
