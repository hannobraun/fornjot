use fj_interop::TriMesh;
use glam::Vec3;
use wgpu::util::DeviceExt;

use super::vertex::Vertex;

pub struct Geometry {
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub num_indices: u32,
}

impl Geometry {
    pub fn new(device: &wgpu::Device, tri_mesh: &TriMesh) -> Self {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();

        for triangle in tri_mesh.all_triangles() {
            let triangle = triangle.points.each_ref().map(|point| {
                Vec3::from(
                    point
                        .coords
                        .components
                        .map(|coord| coord.into_f64() as f32),
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
                let vertex = Vertex {
                    position: point.into(),
                    normal: normal.into(),
                };

                indices.push(index);
                vertices.push(vertex);
            }
        }

        let Ok(num_indices) = indices.len().try_into() else {
            panic!("Unsupported number of indices: `{}`", indices.len());
        };

        let vertices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let indices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertices,
            indices,
            num_indices,
        }
    }
}
