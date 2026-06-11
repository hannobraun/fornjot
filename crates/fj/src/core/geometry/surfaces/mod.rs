//! # Geometry code specific to various types of surfaces

mod swept_curve;
mod transformed_surface;

pub use self::{
    swept_curve::SweptCurve, transformed_surface::TransformedSurface,
};
