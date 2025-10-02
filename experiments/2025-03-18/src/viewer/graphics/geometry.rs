use std::convert::TryInto;

use fj_math::Point;
use wgpu::util::DeviceExt;

use crate::viewer::graphics::{
    RenderMode,
    text::{Label, TextRenderer},
};

use super::vertices::Vertex;

#[derive(Debug)]
pub struct Geometry {
    pub render_mode: RenderMode,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub labels: Vec<Label>,
}

impl Geometry {
    pub fn new(
        render_mode: RenderMode,
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
        text_renderer: &mut TextRenderer,
    ) -> Self {
        Self {
            render_mode,
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
            labels: vec![
                text_renderer
                    .make_label("Hello, world!", Point::from([0., 0., 0.])),
            ],
        }
    }
}
