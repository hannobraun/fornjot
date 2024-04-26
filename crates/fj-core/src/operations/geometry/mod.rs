//! Operations to update the geometry of objects

mod curve;
mod half_edge;

pub use self::{curve::UpdateCurveGeometry, half_edge::UpdateHalfEdgeGeometry};
