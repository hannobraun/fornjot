use std::collections::BTreeMap;

use bytemuck::{Pod, Zeroable};
use fj_interop::{Index, TriMesh};

#[derive(Debug)]
pub struct Vertices {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Vertices {
    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn from_tri_mesh(tri_mesh: &TriMesh) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut indices_by_vertex = BTreeMap::new();

        for triangle in tri_mesh.triangles() {
            let [a, b, c] = triangle.inner.points;

            let normal = (b - a).cross(&(c - a)).normalize();
            let color = triangle.color;

            for point in [a, b, c] {
                let index = *indices_by_vertex
                    .entry((point, normal, color))
                    .or_insert_with(|| {
                        let index = vertices.len();
                        vertices.push(Vertex {
                            position: point.into(),
                            normal: normal.into(),
                            color: color.0.map(|v| f32::from(v) / 255.0),
                        });
                        index as u32
                    });

                indices.push(index);
            }
        }

        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}
