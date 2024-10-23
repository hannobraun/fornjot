//! Geometry that is applied to the topological object graph

pub mod curves;
pub mod surfaces;
pub mod traits;
pub mod util;

mod boundary;
mod geometry;
mod path;
mod tolerance;
mod vertex;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    geometry::{
        CurveGenerator, CurveGeom, Geometry, LocalCurveGeom, SurfaceGeom,
    },
    path::Path,
    tolerance::{InvalidTolerance, Tolerance},
    vertex::{LocalVertexGeom, VertexGeom},
};
