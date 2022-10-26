//! Rendering primitives, routines, and structures.

mod draw_config;
mod drawables;
mod geometries;
mod pipelines;
mod renderer;
mod shaders;
mod transform;
mod uniforms;
mod vertices;

pub use self::{
    draw_config::DrawConfig,
    renderer::{DrawError, Renderer, RendererInitError},
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const SAMPLE_COUNT: u32 = 4;
