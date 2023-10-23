//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left out, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to deal with duplicate vertices.

use crate::objects::{HalfEdge, Surface};

use super::{
    curve::{approx_curve, CurveApproxCache},
    vertex::VertexApproxCache,
    Approx, ApproxPoint, Tolerance,
};

impl Approx for (&HalfEdge, &Surface) {
    type Approximation = HalfEdgeApprox;
    type Cache = EdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (edge, surface) = self;
        let tolerance = tolerance.into();

        let start_position_surface = edge.start_position();
        let start_position = match cache.start_position.get(edge.start_vertex())
        {
            Some(position) => position,
            None => {
                let position_global = surface
                    .geometry()
                    .point_from_surface_coords(start_position_surface);
                cache
                    .start_position
                    .insert(edge.start_vertex().clone(), position_global)
            }
        };

        let first = ApproxPoint::new(start_position_surface, start_position);

        let rest = {
            let cached =
                cache.curve.get(&edge.curve().clone(), edge.boundary());

            let approx = match cached {
                Some(approx) => approx,
                None => {
                    let approx = approx_curve(
                        &edge.path(),
                        surface,
                        edge.boundary(),
                        tolerance,
                    );

                    cache.curve.insert(
                        edge.curve().clone(),
                        edge.boundary(),
                        approx,
                    )
                }
            };

            approx.points.into_iter().map(|point| {
                let point_surface =
                    edge.path().point_from_path_coords(point.local_form);

                ApproxPoint::new(point_surface, point.global_form)
            })
        };

        let mut points = vec![first];
        points.extend(rest);

        HalfEdgeApprox { points }
    }
}

/// An approximation of a [`HalfEdge`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdgeApprox {
    /// The points that approximate the half-edge
    pub points: Vec<ApproxPoint<2>>,
}

/// Cache for edge approximations
#[derive(Default)]
pub struct EdgeApproxCache {
    start_position: VertexApproxCache,
    curve: CurveApproxCache,
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::TAU, ops::Deref};

    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::approx::{Approx, ApproxPoint},
        geometry::{CurveBoundary, GlobalPath, SurfaceGeometry},
        objects::{HalfEdge, Surface},
        operations::BuildHalfEdge,
        services::Services,
    };

    #[test]
    fn approx_line_on_flat_surface() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xz_plane();
        let edge =
            HalfEdge::line_segment([[1., 1.], [2., 1.]], None, &mut services);

        let tolerance = 1.;
        let approx = (&edge, surface.deref()).approx(tolerance);

        let expected_approx = vec![{
            let point_surface = edge.start_position();
            ApproxPoint::from_surface_point(point_surface, &surface)
        }];
        assert_eq!(approx.points, expected_approx);
    }

    #[test]
    fn approx_line_on_curved_surface_but_not_along_curve() {
        let mut services = Services::new();

        let surface = Surface::new(SurfaceGeometry {
            u: GlobalPath::circle_from_radius(1.),
            v: [0., 0., 1.].into(),
        });
        let edge =
            HalfEdge::line_segment([[1., 1.], [2., 1.]], None, &mut services);

        let tolerance = 1.;
        let approx = (&edge, &surface).approx(tolerance);

        let expected_approx = vec![{
            let point_surface = edge.start_position();
            ApproxPoint::from_surface_point(point_surface, &surface)
        }];
        assert_eq!(approx.points, expected_approx);
    }

    #[test]
    fn approx_line_on_curved_surface_along_curve() {
        let mut services = Services::new();

        let path = GlobalPath::circle_from_radius(1.);
        let boundary = CurveBoundary::from([[0.], [TAU]]);

        let surface = Surface::new(SurfaceGeometry {
            u: path,
            v: [0., 0., 1.].into(),
        });
        let edge = HalfEdge::line_segment(
            [[0., 1.], [TAU, 1.]],
            Some(boundary.inner),
            &mut services,
        );

        let tolerance = 1.;
        let approx = (&edge, &surface).approx(tolerance);

        let mut expected_approx = vec![{
            let point_surface = edge.start_position();
            ApproxPoint::from_surface_point(point_surface, &surface)
        }];
        expected_approx.extend(
            (path, boundary).approx(tolerance).into_iter().map(
                |(point_local, _)| {
                    let point_surface =
                        edge.path().point_from_path_coords(point_local);
                    ApproxPoint::from_surface_point(point_surface, &surface)
                },
            ),
        );
        assert_eq!(approx.points, expected_approx);
    }

    #[test]
    fn approx_circle_on_flat_surface() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xz_plane();
        let edge = HalfEdge::circle([0., 0.], 1., &mut services);

        let tolerance = 1.;
        let approx = (&edge, surface.deref()).approx(tolerance);

        let mut expected_approx = vec![{
            let point_surface = edge.start_position();
            ApproxPoint::from_surface_point(point_surface, &surface)
        }];
        expected_approx.extend(
            (&edge.path(), CurveBoundary::from([[0.], [TAU]]))
                .approx(tolerance)
                .into_iter()
                .map(|(_, point_surface)| {
                    ApproxPoint::from_surface_point(point_surface, &surface)
                }),
        );
        assert_eq!(approx.points, expected_approx);
    }
}
