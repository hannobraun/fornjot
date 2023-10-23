//! Types that are tied to objects, but aren't objects themselves

mod boundary;
mod path;
mod surface;

pub use self::{
    boundary::{CurveBoundary, CurveBoundaryElement},
    path::{GlobalPath, SurfacePath},
    surface::SurfaceGeometry,
};
