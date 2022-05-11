//! Intersection algorithms

mod line_segment;
mod surface;

pub use self::{
    line_segment::{line_segment, LineSegmentIntersection},
    surface::surface_surface,
};
