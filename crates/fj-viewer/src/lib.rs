//! Fornjot Model Viewer
//!
//! A model viewer which allows basic navigation and rendering of generated models.

#![warn(missing_docs)]

pub mod camera;
pub mod graphics;
pub mod input;
pub mod run;
pub mod window;

/// Marker describing types implemented with winit.
///
/// See: [Rust by Example: Phantom Types](https://doc.rust-lang.org/stable/rust-by-example/generics/phantom.html)
pub struct Winit();
