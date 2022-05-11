//! Intersection algorithms

mod line_segment;
mod surface_surface;

pub use self::{
    line_segment::{line_segment, LineSegmentIntersection},
    surface_surface::surface_surface,
};
