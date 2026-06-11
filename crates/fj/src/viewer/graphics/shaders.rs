use std::borrow::Cow;

pub struct Shaders {
    module: wgpu::ShaderModule,
}

impl Shaders {
    pub fn new(device: &wgpu::Device) -> Self {
        let module =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "shader.wgsl"
                ))),
            });

        Self { module }
    }

    pub fn face(&self) -> Shader<'_> {
        Shader {
            module: &self.module,
            frag_entry: "frag_face",
        }
    }

    pub fn model(&self) -> Shader<'_> {
        Shader {
            module: &self.module,
            frag_entry: "frag_model",
        }
    }

    pub fn mesh(&self) -> Shader<'_> {
        Shader {
            module: &self.module,
            frag_entry: "frag_mesh",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Shader<'r> {
    pub module: &'r wgpu::ShaderModule,
    pub frag_entry: &'static str,
}
