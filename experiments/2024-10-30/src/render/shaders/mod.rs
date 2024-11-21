use glam::Mat4;

pub struct Shaders {
    pub shader_module: wgpu::ShaderModule,
    pub fragment_targets: [Option<wgpu::ColorTargetState>; 1],
}

impl Shaders {
    pub fn triangles(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let shader_module =
            device.create_shader_module(wgpu::include_wgsl!("triangles.wgsl"));

        let fragment_targets = [Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::all(),
        })];

        Self {
            shader_module,
            fragment_targets,
        }
    }

    pub fn vertex_state(&self) -> wgpu::VertexState {
        wgpu::VertexState {
            module: &self.shader_module,
            entry_point: Some("vertex"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: size_of::<TrianglesVertex>()
                    as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: TrianglesVertex::ATTRIBUTES,
            }],
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

impl TrianglesVertex {
    pub const ATTRIBUTES: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];
}
