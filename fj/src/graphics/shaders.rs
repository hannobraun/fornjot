pub struct Shaders<'r> {
    pub model: Shader<'r>,
    pub mesh: Shader<'r>,
}

pub struct Shader<'r> {
    pub module: &'r wgpu::ShaderModule,
    pub frag_entry: &'static str,
}

impl<'r> Shader<'r> {
    pub fn model(module: &'r wgpu::ShaderModule) -> Self {
        Self {
            module,
            frag_entry: "frag_model",
        }
    }

    pub fn mesh(module: &'r wgpu::ShaderModule) -> Self {
        Self {
            module,
            frag_entry: "frag_mesh",
        }
    }
}
