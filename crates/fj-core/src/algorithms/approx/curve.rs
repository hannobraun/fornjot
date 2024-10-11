use std::collections::BTreeMap;

use fj_math::{Circle, Line, Point};

use crate::{
    geometry::{
        surfaces::SweptCurve, util::tri_mesh::convert_point_surface_to_global,
        CurveBoundary, Geometry, Path, Tolerance,
    },
    storage::Handle,
    topology::{Curve, Surface},
};

use super::{circle::approx_circle, line::approx_line, ApproxPoint};

/// Approximate the provided curve
///
/// The approximation is cached, and cached approximations are used, where
/// possible.
pub fn approx_curve_with_cache(
    curve: &Handle<Curve>,
    surface: &Handle<Surface>,
    boundary: CurveBoundary<Point<1>>,
    tolerance: impl Into<Tolerance>,
    cache: &mut CurveApproxCache,
    geometry: &Geometry,
) -> CurveApprox {
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

fn approx_curve(
    path: &Path<2>,
    surface: &SweptCurve,
    boundary: CurveBoundary<Point<1>>,
    tolerance: impl Into<Tolerance>,
) -> CurveApprox {
    let SweptCurve { u, .. } = surface;
    let points = match (path, u) {
        (Path::Circle(_), Path::Circle(_)) => approx_circle_on_curved_surface(),
        (Path::Circle(circle), Path::Line(_)) => {
            approx_circle_on_straight_surface(
                circle, boundary, surface, tolerance,
            )
        }
        (Path::Line(line), _) => {
            approx_line_on_any_surface(line, boundary, surface, tolerance)
        }
    };

    CurveApprox { points }
}

fn approx_circle_on_curved_surface() -> Vec<ApproxPoint<1>> {
    todo!("Approximating a circle on a curved surface is not supported yet.")
}

fn approx_circle_on_straight_surface(
    circle: &Circle<2>,
    boundary: CurveBoundary<Point<1>>,
    surface: &SweptCurve,
    tolerance: impl Into<Tolerance>,
) -> Vec<ApproxPoint<1>> {
    let tolerance = tolerance.into();

    approx_circle(circle, boundary, tolerance)
        .into_iter()
        .map(|(point_curve, point_surface)| {
            // We're throwing away `point_surface` here, which is a bit weird,
            // as we're recomputing it later (outside of this function).
            //
            // It should be fine though:
            //
            // 1. We're throwing this version away, so there's no danger of
            //    inconsistency between this and the later version.
            // 2. This version should have been computed using the same path and
            //    parameters and the later version will be, so they should be
            //    the same anyway.
            // 3. Not all other cases handled in this function have a surface
            //    point available, so it needs to be computed later anyway, in
            //    the general case.

            let point_global = convert_point_surface_to_global(
                surface,
                point_surface,
                tolerance,
            );
            ApproxPoint::new(point_curve, point_global)
        })
        .collect()
}

fn approx_line_on_any_surface(
    line: &Line<2>,
    boundary: CurveBoundary<Point<1>>,
    surface: &SweptCurve,
    tolerance: impl Into<Tolerance>,
) -> Vec<ApproxPoint<1>> {
    let tolerance = tolerance.into();

    let range_u = CurveBoundary::from(
        boundary
            .inner
            .map(|point_curve| [line.point_from_line_coords(point_curve).u]),
    );

    let SweptCurve { u, .. } = surface;
    let approx_u = match u {
        Path::Circle(circle) => approx_circle(circle, range_u, tolerance),
        Path::Line(line) => approx_line(line),
    };

    let mut points = Vec::new();
    for (u, _) in approx_u {
        let t = (u.t - line.origin().u) / line.direction().u;
        let point_surface = line.point_from_line_coords([t]);
        let point_global =
            convert_point_surface_to_global(surface, point_surface, tolerance);
        points.push(ApproxPoint::new(u, point_global));
    }

    points
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

    use fj_math::{Circle, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::approx::{
            circle::approx_circle, curve::approx_curve, ApproxPoint,
        },
        geometry::{
            surfaces::SweptCurve,
            util::tri_mesh::convert_point_surface_to_global, CurveBoundary,
            Path,
        },
        operations::build::BuildSurface,
        topology::Surface,
        Core,
    };

    #[test]
    fn approx_line_on_flat_surface() {
        let core = Core::new();

        let surface = core.layers.geometry.xz_plane();
        let (path, boundary) = Path::line_from_points([[1., 1.], [2., 1.]]);
        let boundary = CurveBoundary::from(boundary);

        let tolerance = 1.;
        let approx = approx_curve(&path, surface, boundary, tolerance);

        assert_eq!(approx.points, vec![]);
    }

    #[test]
    fn approx_line_on_curved_surface_but_not_along_curve() {
        let surface = SweptCurve {
            u: Path::circle_from_radius(1.),
            v: Vector::from([0., 0., 1.]),
        };
        let (path, boundary) = Path::line_from_points([[1., 1.], [2., 1.]]);
        let boundary = CurveBoundary::from(boundary);

        let tolerance = 1.;
        let approx = approx_curve(&path, &surface, boundary, tolerance);

        assert_eq!(approx.points, vec![]);
    }

    #[test]
    fn approx_line_on_curved_surface_along_curve() {
        let mut core = Core::new();

        let circle = Circle::from_center_and_radius(Point::origin(), 1.);
        let global_path = Path::Circle(circle);
        let surface_geom = SweptCurve {
            u: global_path,
            v: Vector::from([0., 0., 1.]),
        };
        let surface = Surface::from_geometry(surface_geom, &mut core);
        let path = Path::line_from_points_with_coords([
            ([0.], [0., 1.]),
            ([TAU], [TAU, 1.]),
        ]);
        let boundary = CurveBoundary::from([[0.], [TAU]]);

        let tolerance = 1.;
        let approx = approx_curve(&path, &surface_geom, boundary, tolerance);

        let expected_approx = approx_circle(&circle, boundary, tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface = path.point_from_path_coords(point_local);
                let point_global = convert_point_surface_to_global(
                    core.layers.geometry.of_surface(&surface),
                    point_surface,
                    tolerance,
                );
                ApproxPoint::new(point_local, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx.points, expected_approx);
    }

    #[test]
    fn approx_circle_on_flat_surface() {
        let mut core = Core::new();

        let surface_geom = *core.layers.geometry.xz_plane();
        let surface = Surface::from_geometry(surface_geom, &mut core);
        let circle = Circle::from_center_and_radius([0., 0.], 1.);
        let path = Path::Circle(circle);
        let boundary = CurveBoundary::from([[0.], [TAU]]);

        let tolerance = 1.;
        let approx = approx_curve(&path, &surface_geom, boundary, tolerance);

        let expected_approx = approx_circle(&circle, boundary, tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface = path.point_from_path_coords(point_local);
                let point_global = convert_point_surface_to_global(
                    core.layers.geometry.of_surface(&surface),
                    point_surface,
                    tolerance,
                );
                ApproxPoint::new(point_local, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx.points, expected_approx);
    }
}
