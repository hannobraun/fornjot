use std::{collections::HashMap, hash::Hash};

use decorum::R32;

use crate::math::{Point, Vector};

/// API for creating a mesh
pub struct MeshMaker<V> {
    vertices: Vec<V>,
    indices: Vec<Index>,

    indices_by_vertex: HashMap<V, Index>,
}

impl<V> MeshMaker<V>
where
    V: Copy + Eq + Hash,
{
    /// Create a new instance of `MeshMaker`
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            indices_by_vertex: HashMap::new(),
        }
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

/// A point/vector type that can be used as a [`HashMap`] key
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct HashVector(pub [R32; 3]);

impl From<Point> for HashVector {
    fn from(point: Point) -> Self {
        Self([R32::from(point.x), R32::from(point.y), R32::from(point.z)])
    }
}

impl From<Vector> for HashVector {
    fn from(vector: Vector) -> Self {
        Self([
            R32::from(vector.x),
            R32::from(vector.y),
            R32::from(vector.z),
        ])
    }
}

/// An index that refers to a vertex in a mesh
pub type Index = u32;
