use fj_math::{Point, Scalar, Vector};

use crate::{
    geometry::{curve::GlobalPath, surface::SurfaceGeometry},
    partial::PartialSurface,
};

/// Builder API for [`PartialSurface`]
pub trait SurfaceBuilder: Sized {
    /// Build a surface from its two axes
    fn from_axes(u: GlobalPath, v: impl Into<Vector<3>>) -> Self;

    /// Construct a plane from 3 points
    fn update_as_plane_from_points(
        &mut self,
        points: [impl Into<Point<3>>; 3],
    ) -> ([Point<2>; 3], SurfaceGeometry);
}

impl SurfaceBuilder for PartialSurface {
    fn from_axes(u: GlobalPath, v: impl Into<Vector<3>>) -> Self {
        let v = v.into();

        Self {
            geometry: Some(SurfaceGeometry { u, v }),
        }
    }

    fn update_as_plane_from_points(
        &mut self,
        points: [impl Into<Point<3>>; 3],
    ) -> ([Point<2>; 3], SurfaceGeometry) {
        let [a, b, c] = points.map(Into::into);

        let (u, u_coords) = GlobalPath::line_from_points([a, b]);
        let v = c - a;

        let geometry = SurfaceGeometry { u, v };
        self.geometry = Some(geometry);

        let [a, b] = u_coords.map(|point| point.t);
        let points = [[a, Scalar::ZERO], [b, Scalar::ZERO], [a, Scalar::ONE]]
            .map(Point::from);

        (points, geometry)
    }
}
