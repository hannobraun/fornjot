mod pipeline;

pub use self::pipeline::Pipeline;

use super::shaders::{TrianglesVertex, VerticesVertex};

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
        let vertices = Pipeline::vertices(device, config, uniforms);
        let triangles = Pipeline::triangles(device, config, uniforms);

        Self {
            vertices,
            triangles,
        }
    }
}
