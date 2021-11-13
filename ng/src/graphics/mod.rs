mod config_ui;
mod draw_config;
mod drawables;
mod geometries;
mod pipelines;
mod renderer;
mod shaders;
mod transform;
mod uniforms;
// TASK: The name of this module (and the type within it) collides with
//       `util::vertices`. One of the names should be change, and the one in
//       `util` seems pretty much on point for what it does.
mod vertices;

pub use self::{
    draw_config::DrawConfig,
    renderer::{DrawError, Renderer},
    transform::Transform,
};

const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
