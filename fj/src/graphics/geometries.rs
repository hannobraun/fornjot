use std::convert::TryInto;

use wgpu::util::DeviceExt;

use super::{Vertex, Vertices};

#[derive(Debug)]
pub struct Geometries {
    pub mesh: Geometry,
    pub grid: Geometry,
}

impl Geometries {
    pub fn new(device: &wgpu::Device, mesh: &Vertices, grid: Vertices) -> Self {
        let mesh = Geometry::new(device, mesh.vertices(), mesh.indices());
        let grid = Geometry::new(device, grid.vertices(), grid.indices());

        Self { mesh, grid }
    }
}

#[derive(Debug)]
pub struct Geometry {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Geometry {
    fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Self {
        Self {
            vertex_buffer: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ),
            index_buffer: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                },
            ),
            num_indices: indices
                .len()
                .try_into()
                .expect("`usize` couldn't be cast to `u32`"),
        }
    }
}
