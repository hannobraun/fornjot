//! A triangle mesh

use std::{collections::HashMap, hash::Hash};

/// A triangle mesh
pub struct Mesh<V> {
    vertices: Vec<V>,
    indices: Vec<Index>,

    indices_by_vertex: HashMap<V, Index>,
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
    pub fn push(&mut self, vertex: V) {
        let index =
            *self.indices_by_vertex.entry(vertex).or_insert_with(|| {
                let index = self.vertices.len();
                self.vertices.push(vertex);
                index as u32
            });

        self.indices.push(index);
    }

    /// Access the vertices of the mesh
    pub fn vertices(&self) -> impl Iterator<Item = V> + '_ {
        self.vertices.iter().copied()
    }

    /// Access the indices of the mesh
    pub fn indices(&self) -> impl Iterator<Item = Index> + '_ {
        self.indices.iter().copied()
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
        }
    }
}

/// An index that refers to a vertex in a mesh
pub type Index = u32;
