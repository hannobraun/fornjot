//! Loosely coupled layers, that together define shapes
//!
//! See [`Layers`].

mod objects;
mod validation;

mod layer;
mod layers;

pub use self::{
    layer::{Layer, State},
    layers::Layers,
};
