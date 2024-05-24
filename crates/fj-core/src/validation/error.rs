use std::{convert::Infallible, fmt};

use crate::validate::{
    ObjectNotExclusivelyOwned, SketchValidationError, SolidValidationError,
};

use super::checks::{
    AdjacentHalfEdgesNotConnected, CoincidentHalfEdgesAreNotSiblings,
    CurveGeometryMismatch, FaceHasNoBoundary, HalfEdgeHasNoSibling,
    InteriorCycleHasInvalidWinding,
};

/// An error that can occur during a validation
#[derive(Clone, Debug, thiserror::Error)]
pub enum ValidationError {
    /// Adjacent half-edges are not connected
    #[error(transparent)]
    AdjacentHalfEdgesNotConnected(#[from] AdjacentHalfEdgesNotConnected),

    /// Coincident half-edges are not siblings
    #[error(transparent)]
    CoincidentHalfEdgesAreNotSiblings(
        #[from] CoincidentHalfEdgesAreNotSiblings,
    ),

    /// Curve geometry mismatch
    #[error(transparent)]
    CurveGeometryMismatch(#[from] CurveGeometryMismatch),

    /// Face has no boundary
    #[error(transparent)]
    FaceHasNoBoundary(#[from] FaceHasNoBoundary),

    /// Half-edge has no sibling
    #[error(transparent)]
    HalfEdgeHasNoSibling(#[from] HalfEdgeHasNoSibling),

    /// Interior cycle has invalid winding
    #[error(transparent)]
    InteriorCycleHasInvalidWinding(#[from] InteriorCycleHasInvalidWinding),

    /// An object that should be exclusively owned by another, is not
    #[error(transparent)]
    ObjectNotExclusivelyOwned(#[from] ObjectNotExclusivelyOwned),

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

/// A collection of validation errors
#[derive(Debug, thiserror::Error)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_errors = self.0.len();

        writeln!(f, "{num_errors} unhandled validation errors:")?;

        for err in &self.0 {
            writeln!(f, "{err}")?;
        }

        Ok(())
    }
}
