//! Interfaces used when defining models.

mod context;
mod metadata;
mod model;

pub use self::{
    context::Context,
    metadata::{ArgumentMetadata, Metadata, ModelMetadata},
    model::Model,
};

/// A generic error used when defining a model.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
