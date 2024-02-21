use std::convert::Infallible;

use crate::validate::{
    CycleValidationError, EdgeValidationError, FaceValidationError,
    ShellValidationError, SketchValidationError, SolidValidationError,
};

/// An error that can occur during a validation
#[derive(Clone, Debug, thiserror::Error)]
pub enum ValidationError {
    /// `Cycle` validation error
    #[error("`Cycle` validation error")]
    Cycle(#[from] CycleValidationError),

    /// `Edge` validation error
    #[error("`Edge` validation error")]
    Edge(#[from] EdgeValidationError),

    /// `Face` validation error
    #[error("`Face` validation error")]
    Face(#[from] FaceValidationError),

    /// `Shell` validation error
    #[error("`Shell` validation error")]
    Shell(#[from] ShellValidationError),

    /// `Solid` validation error
    #[error("`Solid` validation error")]
    Solid(#[from] SolidValidationError),

    /// `Sketch` validation error
    #[error("`Sketch` validation error")]
    Sketch(#[from] SketchValidationError),
}

impl From<Infallible> for ValidationError {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}
