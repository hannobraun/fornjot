mod pipeline;

pub mod triangles;
pub mod vertices;

use glam::Mat4;

pub use self::pipeline::Pipeline;

pub struct Pipelines {
    pub vertices: Pipeline<vertices::VerticesVertex>,
    pub triangles: Pipeline<triangles::TrianglesVertex>,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniforms: &wgpu::Buffer,
    ) -> Self {
        let vertices = Pipeline::new(
            device,
            config,
            wgpu::include_wgsl!("shaders/vertices.wgsl"),
            uniforms,
        );
        let triangles = Pipeline::new(
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
