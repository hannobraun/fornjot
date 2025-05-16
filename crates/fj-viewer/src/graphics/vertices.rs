use bytemuck::{Pod, Zeroable};
use fj_interop::{Index, TriMesh, vertices_to_indexed_vertices};

#[derive(Debug)]
pub struct Vertices {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Vertices {
    pub fn for_model(tri_mesh: &TriMesh) -> Self {
        let (vertices, indices) = vertices_to_indexed_vertices(
            tri_mesh.triangles.iter().flat_map(|triangle| {
                let [a, b, c] = triangle.inner.points;

                let normal = (b - a).cross(&(c - a)).normalize();
                let color = triangle.color;

                [a, b, c].map(|point| (point, normal, color))
            }),
            |(point, normal, color)| Vertex {
                position: point.into(),
                normal: normal.into(),
                color: color.0.map(|v| f32::from(v) / 255.0),
            },
        );

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
