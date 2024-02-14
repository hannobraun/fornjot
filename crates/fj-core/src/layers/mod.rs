//! Loosely coupled layers, that together define shapes
//!
//! See [`Layers`].

pub mod objects;
pub mod validation;

mod layer;
mod layers;

pub use self::{
    layer::{Layer, State},
    layers::Layers,
};
