//! Types that are tied to objects, but aren't objects themselves

mod path;
mod surface;

pub use self::{
    path::{GlobalPath, SurfacePath},
    surface::SurfaceGeometry,
};
