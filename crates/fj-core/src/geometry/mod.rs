//! Types that are tied to objects, but aren't objects themselves

mod curve;
mod surface;

pub use self::{
    curve::{GlobalPath, SurfacePath},
    surface::SurfaceGeometry,
};
