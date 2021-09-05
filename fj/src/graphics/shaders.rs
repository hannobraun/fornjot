pub struct Shaders<'r> {
    model: Shader<'r>,
    mesh: Shader<'r>,
}

impl<'r> Shaders<'r> {
    pub fn new(module: &'r wgpu::ShaderModule) -> Self {
        Self {
            model: Shader::model(module),
            mesh: Shader::mesh(module),
        }
    }

    pub fn model(&self) -> Shader {
        self.model
    }

    pub fn mesh(&self) -> Shader {
        self.mesh
    }
}

#[derive(Clone, Copy)]
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
