//! Types that are tied to objects, but aren't objects themselves

mod boundary;
mod bounding_vertices;
mod path;
mod surface;

pub use self::{
    boundary::BoundaryOnCurve,
    bounding_vertices::BoundingVertices,
    path::{GlobalPath, SurfacePath},
    surface::SurfaceGeometry,
};
