//! Curve approximation

use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, Geometry, GlobalPath, SurfaceGeom, SurfacePath},
    storage::Handle,
    topology::{Curve, Surface},
};

use super::{
    circle::approx_circle, path::approx_line, Approx, ApproxPoint, Tolerance,
};

impl Approx for (&Handle<Curve>, &Handle<Surface>, CurveBoundary<Point<1>>) {
    type Approximation = CurveApprox;
    type Cache = CurveApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation {
        let (curve, surface, boundary) = self;

        match cache.get(curve, boundary) {
            Some(approx) => approx,
            None => {
                let approx = approx_curve(
                    &geometry
                        .of_curve(curve)
                        .unwrap()
                        .local_on(surface)
                        .unwrap()
                        .path,
                    geometry.of_surface(surface),
                    boundary,
                    tolerance,
                );

                cache.insert(curve.clone(), boundary, approx)
            }
        }
    }
}

fn approx_curve(
    path: &SurfacePath,
    surface: &SurfaceGeom,
    boundary: CurveBoundary<Point<1>>,
    tolerance: impl Into<Tolerance>,
) -> CurveApprox {
    // There are different cases of varying complexity. Circles are the hard
    // part here, as they need to be approximated, while lines don't need to be.
    //
    // This will probably all be unified eventually, as `SurfacePath` and
    // `GlobalPath` grow APIs that are better suited to implementing this code
    // in a more abstract way.
    let points = match (path, surface.u) {
        (SurfacePath::Circle(_), GlobalPath::Circle(_)) => {
            todo!(
                "Approximating a circle on a curved surface not supported yet."
            )
        }
        (SurfacePath::Circle(circle), GlobalPath::Line(_)) => {
            approx_circle(circle, boundary, tolerance)
                .into_iter()
                .map(|(point_curve, point_surface)| {
                    // We're throwing away `point_surface` here, which is a
                    // bit weird, as we're recomputing it later (outside of
                    // this function).
                    //
                    // It should be fine though:
                    //
                    // 1. We're throwing this version away, so there's no
                    //    danger of inconsistency between this and the later
                    //    version.
                    // 2. This version should have been computed using the
                    //    same path and parameters and the later version
                    //    will be, so they should be the same anyway.
                    // 3. Not all other cases handled in this function have
                    //    a surface point available, so it needs to be
                    //    computed later anyway, in the general case.

                    let point_global =
                        surface.point_from_surface_coords(point_surface);
                    ApproxPoint::new(point_curve, point_global)
                })
                .collect()
        }
        (SurfacePath::Line(line), _) => {
            let range_u =
                CurveBoundary::from(boundary.inner.map(|point_curve| {
                    [path.point_from_path_coords(point_curve).u]
                }));

            let approx_u = match surface.u {
                GlobalPath::Circle(circle) => {
                    approx_circle(&circle, range_u, tolerance)
                }
                GlobalPath::Line(line) => approx_line(&line),
            };

            let mut points = Vec::new();
            for (u, _) in approx_u {
                let t = (u.t - line.origin().u) / line.direction().u;
                let point_surface = path.point_from_path_coords([t]);
                let point_global =
                    surface.point_from_surface_coords(point_surface);
                points.push(ApproxPoint::new(u, point_global));
            }

            points
        }
    };

    CurveApprox { points }
}

/// Approximation of a [`Curve`], within a specific boundary
///
/// The approximation of the curve only includes points _within_ the boundary,
/// not those _on_ the boundary. Those boundary points are part of half-edge
/// approximation, which uses and includes curve approximation.
#[derive(Clone)]
pub struct CurveApprox {
    /// The points that approximate the curve within the boundary
    pub points: Vec<ApproxPoint<1>>,
}

impl CurveApprox {
    fn reverse(mut self) -> Self {
        self.points.reverse();
        self
    }
}

/// Cache for curve approximations
#[derive(Default)]
pub struct CurveApproxCache {
    inner: BTreeMap<(Handle<Curve>, CurveBoundary<Point<1>>), CurveApprox>,
}

impl CurveApproxCache {
    fn get(
        &self,
        handle: &Handle<Curve>,
        boundary: CurveBoundary<Point<1>>,
    ) -> Option<CurveApprox> {
        if let Some(approx) = self.inner.get(&(handle.clone(), boundary)) {
            return Some(approx.clone());
        }
        if let Some(approx) =
            self.inner.get(&(handle.clone(), boundary.reverse()))
        {
            return Some(approx.clone().reverse());
        }

        None
    }

    fn insert(
        &mut self,
        handle: Handle<Curve>,
        boundary: CurveBoundary<Point<1>>,
        approx: CurveApprox,
    ) -> CurveApprox {
        self.inner
            .insert((handle, boundary), approx.clone())
            .unwrap_or(approx)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::TAU;

    use fj_math::{Circle, Point};
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::approx::{circle::approx_circle, Approx, ApproxPoint},
        geometry::{CurveBoundary, GlobalPath, SurfacePath},
        operations::build::{BuildCurve, BuildSurface},
        topology::{Curve, Surface},
        Core,
    };

    #[test]
    fn approx_line_on_flat_surface() {
        let mut core = Core::new();

        let surface = core.layers.topology.surfaces.xz_plane();
        let (path, boundary) =
            SurfacePath::line_from_points([[1., 1.], [2., 1.]]);
        let curve =
            Curve::from_path_and_surface(path, surface.clone(), &mut core);
        let boundary = CurveBoundary::from(boundary);

        let tolerance = 1.;
        let approx = (&curve, &surface, boundary)
            .approx(tolerance, &core.layers.geometry);

        assert_eq!(approx.points, vec![]);
    }

    #[test]
    fn approx_line_on_curved_surface_but_not_along_curve() {
        let mut core = Core::new();

        let surface = Surface::from_uv(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
            &mut core,
        );
        let (path, boundary) =
            SurfacePath::line_from_points([[1., 1.], [2., 1.]]);
        let curve =
            Curve::from_path_and_surface(path, surface.clone(), &mut core);
        let boundary = CurveBoundary::from(boundary);

        let tolerance = 1.;
        let approx = (&curve, &surface, boundary)
            .approx(tolerance, &core.layers.geometry);

        assert_eq!(approx.points, vec![]);
    }

    #[test]
    fn approx_line_on_curved_surface_along_curve() {
        let mut core = Core::new();

        let circle = Circle::from_center_and_radius(Point::origin(), 1.);
        let global_path = GlobalPath::Circle(circle);
        let surface = Surface::from_uv(global_path, [0., 0., 1.], &mut core);
        let path = SurfacePath::line_from_points_with_coords([
            ([0.], [0., 1.]),
            ([TAU], [TAU, 1.]),
        ]);
        let curve =
            Curve::from_path_and_surface(path, surface.clone(), &mut core);
        let boundary = CurveBoundary::from([[0.], [TAU]]);

        let tolerance = 1.;
        let approx = (&curve, &surface, boundary)
            .approx(tolerance, &core.layers.geometry);

        let expected_approx = approx_circle(&circle, boundary, tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface = path.point_from_path_coords(point_local);
                let point_global = core
                    .layers
                    .geometry
                    .of_surface(&surface)
                    .point_from_surface_coords(point_surface);
                ApproxPoint::new(point_local, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx.points, expected_approx);
    }

    #[test]
    fn approx_circle_on_flat_surface() {
        let mut core = Core::new();

        let surface = core.layers.topology.surfaces.xz_plane();
        let circle = Circle::from_center_and_radius([0., 0.], 1.);
        let path = SurfacePath::Circle(circle);
        let curve =
            Curve::from_path_and_surface(path, surface.clone(), &mut core);
        let boundary = CurveBoundary::from([[0.], [TAU]]);

        let tolerance = 1.;
        let approx = (&curve, &surface, boundary)
            .approx(tolerance, &core.layers.geometry);

        let expected_approx = approx_circle(&circle, boundary, tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface = path.point_from_path_coords(point_local);
                let point_global = core
                    .layers
                    .geometry
                    .of_surface(&surface)
                    .point_from_surface_coords(point_surface);
                ApproxPoint::new(point_local, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx.points, expected_approx);
    }
}
