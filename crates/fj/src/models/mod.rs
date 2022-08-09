//! Interfaces used when defining models.

mod context;
mod host;
mod metadata;
mod model;

pub use self::{
    context::{
        Context, ContextError, ContextExt, MissingArgument, ParseFailed,
    },
    host::{Host, HostExt},
    metadata::{ArgumentMetadata, Metadata, ModelMetadata},
    model::Model,
};

/// A generic error used when defining a model.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
