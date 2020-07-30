use shaderc::ShaderKind;

pub struct Shaders {
    pub vertex: shaderc::CompilationArtifact,
    pub fragment: shaderc::CompilationArtifact,
}

impl Shaders {
    pub fn compile() -> Result<Self, Error> {
        let mut compiler =
            shaderc::Compiler::new().ok_or(Error::CompilerInit)?;

        let vertex = compiler.compile_into_spirv(
            include_str!("shaders/vert.glsl"),
            ShaderKind::Vertex,
            "vert.glsl",
            "main",
            None,
        )?;
        let fragment = compiler.compile_into_spirv(
            include_str!("shaders/frag.glsl"),
            ShaderKind::Fragment,
            "frag.glsl",
            "main",
            None,
        )?;

        Ok(Self { vertex, fragment })
    }
}

#[derive(Debug)]
pub enum Error {
    CompilerInit,
    Shaderc(shaderc::Error),
}

impl From<shaderc::Error> for Error {
    fn from(err: shaderc::Error) -> Self {
        Self::Shaderc(err)
    }
}
