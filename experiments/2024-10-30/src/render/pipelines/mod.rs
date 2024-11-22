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
        let vertices = Pipeline::<VerticesVertex>::new(
            device,
            config,
            wgpu::include_wgsl!("../shaders/vertices.wgsl"),
            uniforms,
        );
        let triangles = Pipeline::<TrianglesVertex>::new(
            device,
            config,
            wgpu::include_wgsl!("../shaders/triangles.wgsl"),
            uniforms,
        );

        Self {
            vertices,
            triangles,
        }
    }
}
