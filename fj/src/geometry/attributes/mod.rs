//! Attributes of geometry
//!
//! Contains traits and supporting types that define various attributes that
//! geometry can have.

pub mod bounding_volume;
pub mod signed_distance_field;
pub mod surface_normal;
pub mod triangle_mesh;
pub mod vertices;

pub use self::{
    bounding_volume::{Aabb, BoundingVolume},
    signed_distance_field::{Distance, SignedDistanceField},
    surface_normal::SurfaceNormal,
    triangle_mesh::{Mesh, TriangleMesh},
    vertices::Vertices,
};
