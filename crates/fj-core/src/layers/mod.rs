//! Loosely coupled layers, that together define shapes
//!
//! See [`Layers`].

mod layer;
mod layers;
mod objects;
mod validation;

pub use self::{
    layer::{Layer, State},
    layers::Layers,
    objects::InsertObject,
    validation::ValidationEvent,
};
