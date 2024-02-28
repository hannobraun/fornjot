//! Loosely coupled layers, that together define shapes
//!
//! See [`Layers`].

pub mod geometry;
pub mod objects;
pub mod presentation;
pub mod validation;

mod layer;
mod layers;

pub use self::{
    layer::{Command, Event, Layer},
    layers::Layers,
};
