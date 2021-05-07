use crate::mesh;

use super::vertices::{Index, Vertex};

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}

impl From<mesh::Mesh> for Mesh {
    fn from(mesh: mesh::Mesh) -> Self {
        let vertices = mesh
            .vertices()
            .map(|vertex| Vertex {
                position: [
                    vertex.position[0].into_inner(),
                    vertex.position[1].into_inner(),
                    vertex.position[2].into_inner(),
                ],
                normal: [
                    vertex.normal[0].into_inner(),
                    vertex.normal[1].into_inner(),
                    vertex.normal[2].into_inner(),
                ],
            })
            .collect();
        let indices = mesh.indices().collect();

        Self { vertices, indices }
    }
}
