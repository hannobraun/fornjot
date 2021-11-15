use bytemuck::{Pod, Zeroable};
use decorum::R32;

use crate::{
    geometry::faces::Triangle,
    math::Point,
    mesh::{Index, MeshMaker},
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

            let v0 = Point::from(v0);
            let v1 = Point::from(v1);
            let v2 = Point::from(v2);

            let normal = (v1 - v0).cross(&(v2 - v0)).normalize();

            let v0 = [R32::from(v0.x), R32::from(v0.y), R32::from(v0.z)];
            let v1 = [R32::from(v1.x), R32::from(v1.y), R32::from(v1.z)];
            let v2 = [R32::from(v2.x), R32::from(v2.y), R32::from(v2.z)];

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
                    vertex[0].into_inner(),
                    vertex[1].into_inner(),
                    vertex[2].into_inner(),
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
