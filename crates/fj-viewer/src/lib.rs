//! # Fornjot Model Viewer
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This library provides a viewer for Fornjot models.
//!
//! [Fornjot]: https://www.fornjot.app/

#![warn(missing_docs)]

mod assets;
mod camera;
mod graphics;
mod input;
mod screen;
mod viewer;

pub use self::{
    graphics::RendererInitError,
    input::InputEvent,
    screen::{NormalizedScreenPosition, Screen, ScreenSize},
    viewer::Viewer,
};
