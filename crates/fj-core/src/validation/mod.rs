//! # Infrastructure for validating objects
//!
//! ## Nomenclature
//!
//! **Validation** is the process of checking that objects meet specific
//! requirements. Each kind of object has its own set of requirements.
//!
//! An object that meets all the requirement for its kind is considered
//! **valid**. An object that does not meet all of them is considered
//! **invalid**. This results in a **validation error**, which is represented by
//! [`ValidationError`].
//!
//! ## Implementation Note
//!
//! This is a new module whose goal is to replace [`crate::validate`]. While
//! this transition is ongoing, both modules will be incomplete.
//!
//! Issues that track the transition:
//!
//! - <https://github.com/hannobraun/fornjot/issues/1713>
//! - <https://github.com/hannobraun/fornjot/issues/2157>

mod config;
mod error;
mod validation;
mod validation_check;

pub use self::{
    config::ValidationConfig,
    error::{ValidationError, ValidationErrors},
    validation::Validation,
    validation_check::ValidationCheck,
};
