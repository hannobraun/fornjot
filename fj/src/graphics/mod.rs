mod drawables;
mod geometries;
mod mesh;
mod pipelines;
mod renderer;
mod shaders;
mod transform;
mod uniforms;

pub use self::{
    mesh::{Index, Mesh, Vertex},
    renderer::{DrawError, Renderer},
    transform::Transform,
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
