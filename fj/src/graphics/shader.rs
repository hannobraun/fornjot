pub struct Shader<'r> {
    pub module: &'r wgpu::ShaderModule,
    pub frag_entry: &'static str,
}
