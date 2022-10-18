//! # Fornjot Model Viewer
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! This library provides a model viewer which allows basic navigation and
//! rendering of generated models.
//!
//! [Fornjot]: https://www.fornjot.app/

#![warn(missing_docs)]

mod camera;
mod graphics;
mod gui;
mod input;
mod screen;

pub use self::{
    camera::Camera,
    graphics::{DrawConfig, Renderer, RendererInitError},
    input::{InputEvent, InputHandler},
    screen::{NormalizedScreenPosition, Screen, ScreenSize},
};
