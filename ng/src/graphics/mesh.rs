use bytemuck::{Pod, Zeroable};
use decorum::R32;

use crate::{
    geometry::faces::Triangle,
    mesh::{HashVector, Index, MeshMaker},
};

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}

impl From<Vec<Triangle>> for Mesh {
    fn from(triangles: Vec<Triangle>) -> Self {
        let mut mesh = MeshMaker::new();

        for triangle in triangles {
            let [v0, v1, v2] = triangle.0;

            let normal = (v1 - v0).cross(&(v2 - v0)).normalize();

            let v0 = HashVector::from(v0);
            let v1 = HashVector::from(v1);
            let v2 = HashVector::from(v2);

            let normal = [
                R32::from(normal.x),
                R32::from(normal.y),
                R32::from(normal.z),
            ];

            mesh.push((v0, normal));
            mesh.push((v1, normal));
            mesh.push((v2, normal));
        }

        let vertices = mesh
            .vertices()
            .map(|(vertex, normal)| Vertex {
                position: [
                    vertex.0[0].into_inner(),
                    vertex.0[1].into_inner(),
                    vertex.0[2].into_inner(),
                ],
                normal: [
                    normal[0].into_inner(),
                    normal[1].into_inner(),
                    normal[2].into_inner(),
                ],
                color: [1.0, 0.0, 0.0, 1.0],
            })
            .collect();

        let indices = mesh.indices().collect();

        Self { vertices, indices }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}
