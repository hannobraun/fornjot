use fj_math::{Line, Point, Vector};

use crate::{
    geometry::{path::GlobalPath, surface::SurfaceGeometry},
    partial::PartialSurface,
};

/// Builder API for [`PartialSurface`]
pub trait SurfaceBuilder {
    /// Build a surface from its two axes
    fn from_axes(u: GlobalPath, v: impl Into<Vector<3>>) -> Self;

    /// Construct a plane from 3 points
    fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self;
}

impl SurfaceBuilder for PartialSurface {
    fn from_axes(u: GlobalPath, v: impl Into<Vector<3>>) -> Self {
        let v = v.into();

        Self {
            geometry: Some(SurfaceGeometry { u, v }),
        }
    }

    fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);

        let u = GlobalPath::Line(Line::from_points([a, b]));
        let v = c - a;

        Self {
            geometry: Some(SurfaceGeometry { u, v }),
        }
    }
}
