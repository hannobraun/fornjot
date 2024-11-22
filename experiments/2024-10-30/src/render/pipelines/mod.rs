mod pipeline;

use glam::Mat4;

pub use self::pipeline::Pipeline;

use self::pipeline::Vertex;

pub struct Pipelines {
    pub vertices: Pipeline<VerticesVertex>,
    pub triangles: Pipeline<TrianglesVertex>,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniforms: &wgpu::Buffer,
    ) -> Self {
        let vertices = Pipeline::<VerticesVertex>::new(
            device,
            config,
            wgpu::include_wgsl!("shaders/vertices.wgsl"),
            uniforms,
        );
        let triangles = Pipeline::<TrianglesVertex>::new(
            device,
            config,
            wgpu::include_wgsl!("shaders/triangles.wgsl"),
            uniforms,
        );

        Self {
            vertices,
            triangles,
        }
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub transform: Mat4,
    pub transform_for_normals: Mat4,
}

impl Uniforms {
    pub fn from_transform(transform: Mat4) -> Self {
        let transform_for_normals = transform.inverse().transpose();

        Self {
            transform,
            transform_for_normals,
        }
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct TrianglesVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex for TrianglesVertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];
}

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
