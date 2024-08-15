//! Geometry that is applied to the topological object graph

mod boundary;
mod curve;
mod geometry;
mod path;
mod surface;
mod vertex;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    curve::{CurveGeom, LocalCurveGeom},
    geometry::Geometry,
    path::Path,
    surface::SurfaceGeom,
    vertex::{LocalVertexGeom, VertexGeom},
};
