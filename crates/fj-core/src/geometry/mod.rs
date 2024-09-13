//! Geometry that is applied to the topological object graph

mod boundary;
mod curve;
mod geometry;
mod path;
mod surface;
mod tolerance;
mod vertex;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    curve::{CurveGeom, GenPolyline, LocalCurveGeom},
    geometry::Geometry,
    path::Path,
    surface::SurfaceGeom,
    tolerance::{InvalidTolerance, Tolerance},
    vertex::{LocalVertexGeom, VertexGeom},
};
