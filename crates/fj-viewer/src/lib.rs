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
mod display;
mod graphics;
mod input;
mod screen;
mod viewer;
mod window;

pub use self::{
    display::{Error, display},
    graphics::RendererInitError,
    screen::{NormalizedScreenPosition, Screen, ScreenSize},
};
