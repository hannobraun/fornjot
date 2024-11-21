use glam::Vec3;
use wgpu::util::DeviceExt;

use crate::geometry::Operation;

use super::shaders::TrianglesVertex;

pub struct Geometry {
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub num_indices: u32,
}

impl Geometry {
    pub fn triangles(
        device: &wgpu::Device,
        operation: &impl Operation,
    ) -> Self {
        let mut mesh_triangles = Vec::new();
        operation.triangles(&mut mesh_triangles);

        let mut indices = Vec::new();
        let mut vertices = Vec::new();

        for triangle in &mesh_triangles {
            let triangle = triangle.vertices.map(|vertex| {
                Vec3::from(
                    vertex.point.coords.map(|coord| coord.value() as f32),
                )
            });
            let normal = {
                let [a, b, c] = triangle;

                let ab = b - a;
                let ac = c - a;

                ab.cross(ac)
            };

            for point in triangle {
                let index = vertices.len() as u32;
                let vertex = TrianglesVertex {
                    position: point.into(),
                    normal: normal.into(),
                };

                indices.push(index);
                vertices.push(vertex);
            }
        }

        Self::new(device, &vertices, &indices)
    }

    pub fn new(
        device: &wgpu::Device,
        vertices: &[TrianglesVertex],
        indices: &[u32],
    ) -> Self {
        let Ok(num_indices) = indices.len().try_into() else {
            panic!("Unsupported number of indices: `{}`", indices.len());
        };

        let vertices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let indices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertices,
            indices,
            num_indices,
        }
    }
}