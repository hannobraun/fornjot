use std::marker::PhantomData;

use glam::Vec3;
use wgpu::util::DeviceExt;

use crate::geometry::Operation;

use super::shaders::{TrianglesVertex, VerticesVertex};

pub struct Geometry<V> {
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub num_indices: u32,
    _vertex: PhantomData<V>,
}

impl Geometry<VerticesVertex> {
    pub fn vertices(device: &wgpu::Device, operation: &impl Operation) -> Self {
        let mut mesh_vertices = Vec::new();
        operation.vertices(&mut mesh_vertices);

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for mesh_vertex in mesh_vertices {
            let s = 0.05;

            let p = mesh_vertex.point;
            let [a, b, c, d] = [[-s, -s], [s, -s], [-s, s], [s, s]]
                .map(|[x, y]| p + [x, y, 0.])
                .map(|point| {
                    point.coords.components.map(|scalar| scalar.value() as f32)
                });

            for vertex in [a, b, c, c, b, d] {
                let index = vertices.len() as u32;

                let vertex = VerticesVertex {
                    position: vertex,
                    center: p.coords.components.map(|s| s.value() as f32),
                    radius: s as f32,
                };

                vertices.push(vertex);
                indices.push(index);
            }
        }

        Self::new(device, &vertices, &indices)
    }
}

impl Geometry<TrianglesVertex> {
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
                    vertex
                        .point
                        .coords
                        .components
                        .map(|coord| coord.value() as f32),
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
}

impl<V> Geometry<V> {
    pub fn new(device: &wgpu::Device, vertices: &[V], indices: &[u32]) -> Self
    where
        V: bytemuck::NoUninit,
    {
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
            _vertex: PhantomData,
        }
    }
}
