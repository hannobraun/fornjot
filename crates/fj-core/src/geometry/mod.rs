//! Types that are tied to objects, but aren't objects themselves

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
    path::{GlobalPath, SurfacePath},
    surface::SurfaceGeom,
    vertex::{LocalVertexGeom, VertexGeom},
};
