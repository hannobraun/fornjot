use bytemuck::{Pod, Zeroable};
use nalgebra::{vector, Point};
use parry3d_f64::shape::Triangle;

use crate::{
    debug::DebugInfo,
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

    pub fn push_line(
        &mut self,
        line: [Point<f64, 3>; 2],
        normal: [f32; 3],
        color: [f32; 4],
    ) {
        let line = line.into_iter().map(|point| Vertex {
            position: [point.x as f32, point.y as f32, point.z as f32],
            normal,
            color,
        });

        self.vertices.extend(line);

        self.indices.push(self.indices.len() as u32);
        self.indices.push(self.indices.len() as u32);
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
            let normal = [0.; 3];

            let red = [1., 0., 0., 1.];
            let green = [0., 1., 0., 1.];

            let color = if triangle_edge_check.hits.len() % 2 == 0 {
                red
            } else {
                green
            };

            self_.push_line(
                [
                    triangle_edge_check.ray.origin,
                    triangle_edge_check.ray.origin
                        + triangle_edge_check.ray.dir,
                ],
                normal,
                color,
            );

            for &hit in &triangle_edge_check.hits {
                let point = triangle_edge_check.ray.point_at(hit);

                let d = 0.05;
                let color = [0., 0., 0., 1.];

                self_.push_line(
                    [point - vector![d, 0., 0.], point + vector![d, 0., 0.]],
                    normal,
                    color,
                );
                self_.push_line(
                    [point - vector![0., d, 0.], point + vector![0., d, 0.]],
                    normal,
                    color,
                );
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
