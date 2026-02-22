use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, Geometry, LocalCurveGeom, Path},
    interop::ext::ArrayExt,
    layers::Layer,
    storage::Handle,
    topology::{Curve, Surface},
};

/// Update the geometry of a [`Curve`]
pub trait UpdateCurveGeometry {
    /// Copy the geometry of the provided curve
    fn copy_geometry_from(
        self,
        curve: &Handle<Curve>,
        geometry: &mut Layer<Geometry>,
    ) -> Self;

    /// Define the geometry as a path on a surface
    fn make_path_on_surface(
        self,
        path: Path<2>,
        surface: Handle<Surface>,
        geometry: &mut Layer<Geometry>,
    ) -> Self;

    /// Define the geometry as a line
    ///
    /// The line is constructed from two points on the provided surface.
    ///
    /// Optionally the coordinates of those points on the curve can be supplied.
    /// If those are not provided, it is assumed that the provided surface
    /// points have the curve coordinates `0` and `1`.
    fn make_line_on_surface(
        self,
        points_surface: [impl Into<Point<2>>; 2],
        points_curve: CurveBoundary<Point<1>>,
        surface: Handle<Surface>,
        geometry: &mut Layer<Geometry>,
    ) -> Self;
}

impl UpdateCurveGeometry for Handle<Curve> {
    fn copy_geometry_from(
        self,
        curve: &Handle<Curve>,
        geometry: &mut Layer<Geometry>,
    ) -> Self {
        let curve_geom = geometry.of_curve(curve).cloned();
        if let Some(curve_geom) = curve_geom {
            for (surface, local_definition) in curve_geom.definitions {
                geometry.define_curve(self.clone(), surface, local_definition);
            }
        }

        self
    }

    fn make_path_on_surface(
        self,
        path: Path<2>,
        surface: Handle<Surface>,
        geometry: &mut Layer<Geometry>,
    ) -> Self {
        geometry.define_curve(self.clone(), surface, LocalCurveGeom { path });

        self
    }

    fn make_line_on_surface(
        self,
        points_surface: [impl Into<Point<2>>; 2],
        points_curve: CurveBoundary<Point<1>>,
        surface: Handle<Surface>,
        geometry: &mut Layer<Geometry>,
    ) -> Self {
        let path = Path::line_from_points_with_coords(
            points_curve.inner.zip_ext(points_surface),
        );

        self.make_path_on_surface(path, surface, geometry)
    }
}
