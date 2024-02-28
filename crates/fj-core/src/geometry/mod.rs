//! Types that are tied to objects, but aren't objects themselves

mod boundary;
mod geometry;
mod path;
mod surface;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    geometry::Geometry,
    path::{GlobalPath, SurfacePath},
    surface::SurfaceGeometry,
};
