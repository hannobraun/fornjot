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
        Shader {
            module: &self.0,
            frag_entry: "frag_model",
        }
    }

    pub fn mesh(&self) -> Shader {
        Shader {
            module: &self.0,
            frag_entry: "frag_mesh",
        }
    }

    pub fn lines(&self) -> Shader {
        Shader {
            module: &self.0,
            frag_entry: "frag_lines",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Shader<'r> {
    pub module: &'r wgpu::ShaderModule,
    pub frag_entry: &'static str,
}
