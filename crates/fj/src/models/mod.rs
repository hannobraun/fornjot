//! Interfaces used when defining models.

mod context;
mod metadata;
mod model;

pub use self::{
    context::Context,
    metadata::{ArgumentMetadata, Metadata, ModelMetadata},
    model::Model,
};
