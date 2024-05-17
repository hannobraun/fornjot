//! All validation checks
//!
//! See documentation of [parent module](super) for more information.

mod curve_geometry_mismatch;
mod face_boundary;
mod face_winding;
mod half_edge_connection;

pub use self::{
    curve_geometry_mismatch::CurveGeometryMismatch,
    face_boundary::FaceHasNoBoundary,
    face_winding::InteriorCycleHasInvalidWinding,
    half_edge_connection::AdjacentHalfEdgesNotConnected,
};
