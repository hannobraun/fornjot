use glam::Mat4;

use super::Pipeline;

pub struct Pipelines {
    pub triangles: Pipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let triangles = Pipeline::new(device, surface_configuration);

        Self { triangles }
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
