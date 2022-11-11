use fj_math::Vector;

use crate::{
    geometry::{path::GlobalPath, surface::SurfaceGeometry},
    partial::PartialSurface,
};

/// Builder API for [`PartialSurface`]
pub trait SurfaceBuilder {
    /// Build a surface from its two axes
    fn from_axes(u: GlobalPath, v: impl Into<Vector<3>>) -> Self;
}

impl SurfaceBuilder for PartialSurface {
    fn from_axes(u: GlobalPath, v: impl Into<Vector<3>>) -> Self {
        let v = v.into();

        Self {
            geometry: Some(SurfaceGeometry { u, v }),
        }
    }
}
