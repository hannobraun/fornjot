use std::collections::BTreeMap;

use bytemuck::{Pod, Zeroable};
use fj_interop::{Color, Index, Mesh};
use fj_math::{Point, Vector};

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

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}

impl From<&Mesh<fj_math::Point<3>>> for Vertices {
    fn from(mesh: &Mesh<fj_math::Point<3>>) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut indices_by_vertex = BTreeMap::new();

        for triangle in mesh.triangles() {
            let [a, b, c] = triangle.inner.points;

            let normal = (b - a).cross(&(c - a)).normalize();
            let color = triangle.color;

            push_vertex(
                &mut vertices,
                &mut indices,
                &mut indices_by_vertex,
                (a, normal, color),
            );
            push_vertex(
                &mut vertices,
                &mut indices,
                &mut indices_by_vertex,
                (b, normal, color),
            );
            push_vertex(
                &mut vertices,
                &mut indices,
                &mut indices_by_vertex,
                (c, normal, color),
            );
        }

        let vertices = vertices
            .into_iter()
            .map(|(vertex, normal, color)| Vertex {
                position: vertex.into(),
                normal: normal.into(),
                color: color.0.map(|v| f32::from(v) / 255.0),
            })
            .collect();

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

fn push_vertex(
    vertices: &mut Vec<(Point<3>, Vector<3>, Color)>,
    indices: &mut Vec<Index>,
    indices_by_vertex: &mut BTreeMap<(Point<3>, Vector<3>, Color), Index>,
    vertex: (Point<3>, Vector<3>, Color),
) {
    let index = *indices_by_vertex.entry(vertex).or_insert_with(|| {
        let index = vertices.len();
        vertices.push(vertex);
        index as u32
    });

    indices.push(index);
}
