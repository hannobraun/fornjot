//! Attributes of geometry
//!
//! Contains traits and supporting types that define various attributes that
//! geometry can have.

pub mod bounding_volume;
pub mod edges;
pub mod edges2;
pub mod path;
pub mod signed_distance_field;
pub mod surface_mesh;
pub mod surface_normal;
pub mod vertices;

pub use self::{
    bounding_volume::{Aabb, BoundingVolume},
    edges::Edges,
    edges2::Edges2,
    path::Path,
    signed_distance_field::{Distance, SignedDistanceField},
    surface_mesh::SurfaceMesh,
    surface_normal::SurfaceNormal,
    vertices::Vertices,
};
