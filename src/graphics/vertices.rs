use bytemuck::{Pod, Zeroable};
use parry3d_f64::shape::Triangle;

use crate::{
    debug::DebugInfo,
    math::Point,
    mesh::{HashVector, Index, MeshMaker},
};

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

impl From<&Vec<Triangle>> for Vertices {
    fn from(triangles: &Vec<Triangle>) -> Self {
        let mut mesh = MeshMaker::new();

        for triangle in triangles {
            let [a, b, c] = triangle.vertices();

            let normal = (b - a).cross(&(c - a)).normalize();

            let a = HashVector::from(a);
            let b = HashVector::from(b);
            let c = HashVector::from(c);

            let normal = HashVector::from(&normal);

            mesh.push((a, normal));
            mesh.push((b, normal));
            mesh.push((c, normal));
        }

        let vertices = mesh
            .vertices()
            .map(|(vertex, normal)| Vertex {
                position: vertex.into(),
                normal: normal.into(),
                color: [1.0, 0.0, 0.0, 1.0],
            })
            .collect();

        let indices = mesh.indices().collect();

        Self { vertices, indices }
    }
}

impl From<&DebugInfo> for Vertices {
    fn from(debug_info: &DebugInfo) -> Self {
        let mut self_ = Self::empty();

        for triangle_edge_check in &debug_info.triangle_edge_checks {
            let red = [1., 0., 0., 1.];
            let green = [0., 1., 0., 1.];

            let color = if triangle_edge_check.hits.len() % 2 == 0 {
                red
            } else {
                green
            };

            let line = line(
                triangle_edge_check.ray.origin,
                triangle_edge_check.ray.origin + triangle_edge_check.ray.dir,
                color,
            );

            self_.vertices.extend(line);

            self_.indices.push(self_.indices.len() as u32);
            self_.indices.push(self_.indices.len() as u32);

            fn line(a: Point, b: Point, color: [f32; 4]) -> [Vertex; 2] {
                [vertex(a, color), vertex(b, color)]
            }

            fn vertex(pos: Point, color: [f32; 4]) -> Vertex {
                Vertex {
                    position: [pos.x as f32, pos.y as f32, pos.z as f32],
                    normal: [0.; 3],
                    color,
                }
            }
        }

        self_
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}
