//! # Fornjot Window Abstraction
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This library provides a window abstraction based on Winit.
//!
//! [Fornjot]: https://www.fornjot.app/

mod display;
mod window;

pub use self::{
    display::{Error, display},
    window::WindowError,
};
