use super::pipeline::Vertex;

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct VerticesVertex {
    pub position: [f32; 3],
    pub center: [f32; 3],
    pub radius: f32,
}

impl Vertex for VerticesVertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32,
    ];
}
