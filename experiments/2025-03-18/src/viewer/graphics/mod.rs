//! Rendering primitives, routines, and structures.

mod device;
mod draw_config;
mod geometry;
mod model;
mod navigation_cube;
mod pipelines;
mod renderer;
mod shaders;
mod text;
mod texture;
mod transform;
mod uniforms;
mod vertices;

pub use self::{
    device::DeviceError,
    draw_config::DrawConfig,
    pipelines::RenderMode,
    renderer::{Renderer, RendererInitError},
    vertices::Vertices,
};

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const SAMPLE_COUNT: u32 = 4;
