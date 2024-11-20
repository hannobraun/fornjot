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
