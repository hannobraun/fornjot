//! Rendering primitives, routines, and structures.

mod config_ui;
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
    renderer::{DrawError, InitError, Renderer},
    vertices::Vertices,
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
