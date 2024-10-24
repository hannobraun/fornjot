//! Geometry that is applied to the topological object graph

pub mod curves;
pub mod repr;
pub mod surfaces;
pub mod traits;

mod boundary;
mod geometry;
mod path;
mod tolerance;
mod vertex;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    geometry::{
        CurveGeom, CurveGeom2, Geometry, LocalCurveGeom, SurfaceGenerator,
    },
    path::Path,
    tolerance::{InvalidTolerance, Tolerance},
    vertex::{LocalVertexGeom, VertexGeom},
};
