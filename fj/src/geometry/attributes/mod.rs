//! Attributes of geometry
//!
//! Contains traits and supporting types that define various attributes that
//! geometry can have.

pub mod bounding_volume;
pub mod signed_distance_field;
pub mod surface_normal;

pub use self::{
    bounding_volume::BoundingVolume,
    signed_distance_field::{Distance, SignedDistanceField},
    surface_normal::SurfaceNormal,
};
