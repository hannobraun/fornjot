//! All validation checks
//!
//! See documentation of [parent module](super) for more information.

mod face_boundary;
mod half_edge_connection;

pub use self::{
    face_boundary::FaceHasNoBoundary,
    half_edge_connection::AdjacentHalfEdgesNotConnected,
};
