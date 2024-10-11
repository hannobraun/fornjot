//! Geometry that is applied to the topological object graph

pub mod curves;
pub mod traits;
pub mod util;

mod boundary;
mod curve;
mod geometry;
mod path;
mod surface;
mod tolerance;
mod vertex;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    curve::{CurveGeom, CurveGeom2, LocalCurveGeom},
    geometry::Geometry,
    path::Path,
    surface::SweptCurve,
    tolerance::{InvalidTolerance, Tolerance},
    vertex::{LocalVertexGeom, VertexGeom},
};
