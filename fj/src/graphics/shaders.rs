use std::borrow::Cow;

pub struct Shaders(wgpu::ShaderModule);

impl Shaders {
    pub fn new(device: &wgpu::Device) -> Self {
        let module =
            device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "shader.wgsl"
                ))),
            });

        Self(module)
    }

    pub fn model(&self) -> Shader {
        Shader::model(&self.0)
    }

    pub fn mesh(&self) -> Shader {
        Shader::mesh(&self.0)
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
