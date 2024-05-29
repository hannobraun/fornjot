//! All validation checks
//!
//! See documentation of [parent module](super) for more information.

mod coincident_half_edges_are_not_siblings;
mod curve_geometry_mismatch;
mod face_boundary;
mod face_winding;
mod half_edge_connection;
mod half_edge_has_no_sibling;
mod multiple_references;

pub use self::{
    coincident_half_edges_are_not_siblings::CoincidentHalfEdgesAreNotSiblings,
    curve_geometry_mismatch::CurveGeometryMismatch,
    face_boundary::FaceHasNoBoundary,
    face_winding::InteriorCycleHasInvalidWinding,
    half_edge_connection::AdjacentHalfEdgesNotConnected,
    half_edge_has_no_sibling::HalfEdgeHasNoSibling,
    multiple_references::MultipleReferencesToObject,
};
