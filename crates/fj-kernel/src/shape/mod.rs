//! The API used for creating and manipulating shapes
//!
//! See [`Shape`], which is the main entry point to this API.

mod api;
mod local;
mod object;
mod stores;
mod update;
mod validate;

pub use self::{
    api::Shape,
    local::LocalForm,
    object::Object,
    stores::{Handle, Iter},
    update::Update,
    validate::{StructuralIssues, ValidationError, ValidationResult},
};
