//! Fornjot Model Viewer
//!
//! A model viewer which allows basic navigation and rendering of generated models.

#![warn(missing_docs)]

pub mod camera;
pub mod graphics;
pub mod input;
pub mod run;
pub mod window;

/// Marker type describing types implemented with `winit`.
///
#[doc = include_str!("../docs/phantom_type.md")]
pub struct Winit();
