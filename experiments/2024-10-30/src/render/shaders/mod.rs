use glam::Mat4;

pub struct Shaders {
    pub shader_module: wgpu::ShaderModule,
}

impl Shaders {
    pub fn triangles(device: &wgpu::Device) -> Self {
        let shader_module =
            device.create_shader_module(wgpu::include_wgsl!("triangles.wgsl"));

        Self { shader_module }
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
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}
