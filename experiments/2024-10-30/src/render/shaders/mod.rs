use std::marker::PhantomData;

use glam::Mat4;

pub struct Shaders<V> {
    pub shader_module: wgpu::ShaderModule,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub fragment_targets: [Option<wgpu::ColorTargetState>; 1],
    _vertex: PhantomData<V>,
}

impl<V> Shaders<V> {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader_module: wgpu::ShaderModule,
    ) -> Self {
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let fragment_targets = [Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::all(),
        })];

        Self {
            shader_module,
            bind_group_layout,
            fragment_targets,
            _vertex: PhantomData,
        }
    }

    pub fn vertex_state(&self) -> wgpu::VertexState
    where
        V: Vertex,
    {
        wgpu::VertexState {
            module: &self.shader_module,
            entry_point: Some("vertex"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: size_of::<V>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: V::ATTRIBUTES,
            }],
        }
    }

    pub fn fragment_state(&self) -> wgpu::FragmentState {
        wgpu::FragmentState {
            module: &self.shader_module,
            entry_point: Some("fragment"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &self.fragment_targets,
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

pub trait Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute];
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
