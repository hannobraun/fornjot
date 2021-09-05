use std::convert::TryInto;

use wgpu::util::DeviceExt;

use super::mesh::Mesh;

#[derive(Debug)]
pub struct Geometry {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Geometry {
    pub fn mesh(device: &wgpu::Device, mesh: &Mesh) -> Self {
        Geometry {
            vertex_buffer: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(mesh.vertices()),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ),
            index_buffer: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(mesh.indices()),
                    usage: wgpu::BufferUsages::INDEX,
                },
            ),
            num_indices: mesh
                .indices()
                .len()
                .try_into()
                .expect("`usize` couldn't be cast to `u32`"),
        }
    }
}
