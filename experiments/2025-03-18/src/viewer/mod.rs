//! # Fornjot Model Viewer
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This library provides a viewer for Fornjot models.
//!
//! [Fornjot]: https://www.fornjot.app/

mod assets;
mod camera;
mod graphics;
mod input;
mod viewer;
mod window;

pub use self::{
    graphics::RendererInitError,
    viewer::{Error, ViewerHandle, WindowHandle, make_viewer_and_spawn_thread},
};
