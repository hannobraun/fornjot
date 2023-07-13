//! Types that are tied to objects, but aren't objects themselves

pub mod curve;
pub mod surface;

pub use self::{
    curve::{GlobalPath, SurfacePath},
    surface::SurfaceGeometry,
};
