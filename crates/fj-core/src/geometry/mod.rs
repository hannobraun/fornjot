//! Types that are tied to objects, but aren't objects themselves

mod boundary;
mod geometry;
mod half_edge;
mod path;
mod surface;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    geometry::Geometry,
    half_edge::HalfEdgeGeom,
    path::{GlobalPath, SurfacePath},
    surface::SurfaceGeometry,
};
