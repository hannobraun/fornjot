//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left off, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to call with duplicate vertices.

use crate::{
    objects::{HalfEdge, Surface},
    storage::Handle,
};

use super::{
    curve::{CurveApprox, CurveCache},
    path::RangeOnPath,
    Approx, ApproxPoint, Tolerance,
};

impl Approx for (&Handle<HalfEdge>, &Surface) {
    type Approximation = HalfEdgeApprox;
    type Cache = CurveCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (half_edge, surface) = self;

        let boundary = half_edge.boundary();
        let range = RangeOnPath { boundary };

        let first = ApproxPoint::new(
            half_edge.start_vertex().position(),
            half_edge.start_vertex().global_form().position(),
        )
        .with_source((half_edge.clone(), half_edge.boundary()[0]));
        let curve_approx = (
            half_edge.curve(),
            surface,
            half_edge.global_form().curve().clone(),
            range,
        )
            .approx_with_cache(tolerance, cache);

        HalfEdgeApprox {
            first,
            curve_approx,
        }
    }
}

/// An approximation of an [`HalfEdge`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdgeApprox {
    /// The point that approximates the first vertex of the curve
    pub first: ApproxPoint<2>,

    /// The approximation of the edge's curve
    pub curve_approx: CurveApprox,
}

impl HalfEdgeApprox {
    /// Compute the points that approximate the edge
    pub fn points(&self) -> Vec<ApproxPoint<2>> {
        let mut points = Vec::new();

        points.push(self.first.clone());
        points.extend(self.curve_approx.points.clone());

        points
    }
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::TAU, ops::Deref};

    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::approx::{path::RangeOnPath, Approx, ApproxPoint},
        builder::{CurveBuilder, HalfEdgeBuilder, SurfaceBuilder},
        geometry::path::GlobalPath,
        insert::Insert,
        objects::GlobalCurve,
        partial::{
            PartialCurve, PartialHalfEdge, PartialObject, PartialSurface,
        },
        services::Services,
    };

    use super::CurveApprox;

    #[test]
    fn approx_line_on_flat_surface() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xz_plane();
        let half_edge = {
            let mut half_edge = PartialHalfEdge::default();

            half_edge.update_as_line_segment_from_points([[1., 1.], [2., 1.]]);
            half_edge.infer_vertex_positions_if_necessary(&surface.geometry());

            half_edge
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        let approx = (&half_edge, surface.deref()).approx(1.);

        assert_eq!(approx.curve_approx, CurveApprox::empty());
    }

    #[test]
    fn approx_line_on_curved_surface_but_not_along_curve() {
        let mut services = Services::new();

        let surface = PartialSurface::from_axes(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
        )
        .build(&mut services.objects)
        .insert(&mut services.objects);
        let half_edge = {
            let mut half_edge = PartialHalfEdge::default();

            half_edge.update_as_line_segment_from_points([[1., 1.], [2., 1.]]);
            half_edge.infer_vertex_positions_if_necessary(&surface.geometry());

            half_edge
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        let approx = (&half_edge, surface.deref()).approx(1.);

        assert_eq!(approx.curve_approx, CurveApprox::empty());
    }

    #[test]
    fn approx_line_on_curved_surface_along_curve() {
        let mut services = Services::new();

        let path = GlobalPath::circle_from_radius(1.);
        let surface = PartialSurface::from_axes(path, [0., 0., 1.])
            .build(&mut services.objects)
            .insert(&mut services.objects);
        let mut curve = PartialCurve::default();
        curve.update_as_line_from_points([[0., 1.], [1., 1.]]);
        let curve = curve
            .build(&mut services.objects)
            .insert(&mut services.objects);
        let global_curve = GlobalCurve.insert(&mut services.objects);

        let range = RangeOnPath::from([[0.], [TAU]]);
        let tolerance = 1.;

        let approx =
            (&curve, surface.deref(), global_curve, range).approx(tolerance);

        let expected_approx = (path, range)
            .approx(tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface =
                    curve.path().point_from_path_coords(point_local);
                let point_global =
                    surface.geometry().point_from_surface_coords(point_surface);
                ApproxPoint::new(point_surface, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx, CurveApprox::empty().with_points(expected_approx));
    }

    #[test]
    fn approx_circle_on_flat_surface() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xz_plane();
        let mut curve = PartialCurve::default();
        curve.update_as_circle_from_radius(1.);
        let curve = curve
            .build(&mut services.objects)
            .insert(&mut services.objects);
        let global_curve = GlobalCurve.insert(&mut services.objects);

        let range = RangeOnPath::from([[0.], [TAU]]);
        let tolerance = 1.;
        let approx =
            (&curve, surface.deref(), global_curve, range).approx(tolerance);

        let expected_approx = (curve.path(), range)
            .approx(tolerance)
            .into_iter()
            .map(|(_, point_surface)| {
                let point_global =
                    surface.geometry().point_from_surface_coords(point_surface);
                ApproxPoint::new(point_surface, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx, CurveApprox::empty().with_points(expected_approx));
    }
}
