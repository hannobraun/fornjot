//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left off, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to call with duplicate vertices.

use std::collections::BTreeMap;

use crate::{
    geometry::path::{GlobalPath, SurfacePath},
    objects::{GlobalEdge, HalfEdge, Surface},
    storage::{Handle, ObjectId},
};

use super::{path::RangeOnPath, Approx, ApproxPoint, Tolerance};

impl Approx for (&Handle<HalfEdge>, &Surface) {
    type Approximation = HalfEdgeApprox;
    type Cache = EdgeCache;

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

        let points = {
            let approx = match cache.get(half_edge.global_form().clone(), range)
            {
                Some(approx) => approx,
                None => {
                    let approx = approx_edge(
                        &half_edge.curve(),
                        surface,
                        range,
                        tolerance,
                    );
                    cache.insert(half_edge.global_form().clone(), range, approx)
                }
            };

            approx
                .points
                .into_iter()
                .map(|point| {
                    let point_surface = half_edge
                        .curve()
                        .point_from_path_coords(point.local_form);

                    ApproxPoint::new(point_surface, point.global_form)
                        .with_source((half_edge.clone(), point.local_form))
                })
                .collect()
        };

        HalfEdgeApprox { first, points }
    }
}

/// An approximation of an [`HalfEdge`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdgeApprox {
    /// The point that approximates the first vertex of the edge
    pub first: ApproxPoint<2>,

    /// The approximation of the edge
    pub points: Vec<ApproxPoint<2>>,
}

impl HalfEdgeApprox {
    /// Compute the points that approximate the edge
    pub fn points(&self) -> Vec<ApproxPoint<2>> {
        let mut points = Vec::new();

        points.push(self.first.clone());
        points.extend(self.points.iter().cloned());

        points
    }
}

fn approx_edge(
    curve: &SurfacePath,
    surface: &Surface,
    range: RangeOnPath,
    tolerance: impl Into<Tolerance>,
) -> GlobalEdgeApprox {
    // There are different cases of varying complexity. Circles are the hard
    // part here, as they need to be approximated, while lines don't need to be.
    //
    // This will probably all be unified eventually, as `SurfacePath` and
    // `GlobalPath` grow APIs that are better suited to implementing this code
    // in a more abstract way.
    let points = match (curve, surface.geometry().u) {
        (SurfacePath::Circle(_), GlobalPath::Circle(_)) => {
            todo!(
                "Approximating a circle on a curved surface not supported yet."
            )
        }
        (SurfacePath::Circle(_), GlobalPath::Line(_)) => {
            (curve, range)
                .approx_with_cache(tolerance, &mut ())
                .into_iter()
                .map(|(point_curve, point_surface)| {
                    // We're throwing away `point_surface` here, which is a bit
                    // weird, as we're recomputing it later (outside of this
                    // function).
                    //
                    // It should be fine though:
                    //
                    // 1. We're throwing this version away, so there's no danger
                    //    of inconsistency between this and the later version.
                    // 2. This version should have been computed using the same
                    //    path and parameters and the later version will be, so
                    //    they should be the same anyway.
                    // 3. Not all other cases handled in this function have a
                    //    surface point available, so it needs to be computed
                    //    later anyway, in the general case.

                    let point_global = surface
                        .geometry()
                        .point_from_surface_coords(point_surface);
                    (point_curve, point_global)
                })
                .collect()
        }
        (SurfacePath::Line(line), _) => {
            let range_u =
                RangeOnPath::from(range.boundary.map(|point_curve| {
                    [curve.point_from_path_coords(point_curve).u]
                }));

            let approx_u = (surface.geometry().u, range_u)
                .approx_with_cache(tolerance, &mut ());

            let mut points = Vec::new();
            for (u, _) in approx_u {
                let t = (u.t - line.origin().u) / line.direction().u;
                let point_surface = curve.point_from_path_coords([t]);
                let point_global =
                    surface.geometry().point_from_surface_coords(point_surface);
                points.push((u, point_global));
            }

            points
        }
    };

    let points = points
        .into_iter()
        .map(|(point_curve, point_global)| {
            ApproxPoint::new(point_curve, point_global)
        })
        .collect();
    GlobalEdgeApprox { points }
}

/// A cache for results of an approximation
#[derive(Default)]
pub struct EdgeCache {
    inner: BTreeMap<(ObjectId, RangeOnPath), GlobalEdgeApprox>,
}

impl EdgeCache {
    /// Create an empty cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert the approximation of a [`GlobalEdge`]
    pub fn insert(
        &mut self,
        handle: Handle<GlobalEdge>,
        range: RangeOnPath,
        approx: GlobalEdgeApprox,
    ) -> GlobalEdgeApprox {
        self.inner.insert((handle.id(), range), approx.clone());
        approx
    }

    /// Access the approximation for the given [`GlobalEdge`], if available
    pub fn get(
        &self,
        handle: Handle<GlobalEdge>,
        range: RangeOnPath,
    ) -> Option<GlobalEdgeApprox> {
        if let Some(approx) = self.inner.get(&(handle.id(), range)) {
            return Some(approx.clone());
        }
        if let Some(approx) = self.inner.get(&(handle.id(), range.reverse())) {
            // If we have a cache entry for the reverse range, we need to use
            // that too!
            return Some(approx.clone().reverse());
        }

        None
    }
}

/// An approximation of a [`GlobalEdge`]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalEdgeApprox {
    /// The points that approximate the edge
    pub points: Vec<ApproxPoint<1>>,
}

impl GlobalEdgeApprox {
    /// Reverse the order of the approximation
    pub fn reverse(mut self) -> Self {
        self.points.reverse();
        self
    }
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::TAU, ops::Deref};

    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::approx::{path::RangeOnPath, Approx, ApproxPoint},
        builder::{HalfEdgeBuilder, SurfaceBuilder},
        geometry::path::GlobalPath,
        insert::Insert,
        partial::{PartialHalfEdge, PartialObject, PartialSurface},
        services::Services,
    };

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

        let tolerance = 1.;
        let approx = (&half_edge, surface.deref()).approx(tolerance);

        assert_eq!(approx.points, Vec::new());
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

        let tolerance = 1.;
        let approx = (&half_edge, surface.deref()).approx(tolerance);

        assert_eq!(approx.points, Vec::new());
    }

    #[test]
    fn approx_line_on_curved_surface_along_curve() {
        let mut services = Services::new();

        let path = GlobalPath::circle_from_radius(1.);
        let range = RangeOnPath::from([[0.], [TAU]]);

        let surface = PartialSurface::from_axes(path, [0., 0., 1.])
            .build(&mut services.objects)
            .insert(&mut services.objects);
        let half_edge = {
            let mut half_edge = PartialHalfEdge::default();

            half_edge.update_as_line_segment_from_points([[0., 1.], [1., 1.]]);

            half_edge.vertices[0].0 = Some(range.boundary[0]);
            half_edge.vertices[1].0 = Some(range.boundary[1]);

            half_edge.infer_vertex_positions_if_necessary(&surface.geometry());

            half_edge
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        let tolerance = 1.;
        let approx = (&half_edge, surface.deref()).approx(tolerance);

        let expected_approx = (path, range)
            .approx(tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface =
                    half_edge.curve().point_from_path_coords(point_local);
                let point_global =
                    surface.geometry().point_from_surface_coords(point_surface);
                ApproxPoint::new(point_surface, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx.points, expected_approx);
    }

    #[test]
    fn approx_circle_on_flat_surface() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xz_plane();
        let half_edge = {
            let mut half_edge = PartialHalfEdge::default();

            half_edge.update_as_circle_from_radius(1.);
            half_edge.infer_vertex_positions_if_necessary(&surface.geometry());

            half_edge
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        let tolerance = 1.;
        let approx = (&half_edge, surface.deref()).approx(tolerance);

        let expected_approx =
            (&half_edge.curve(), RangeOnPath::from([[0.], [TAU]]))
                .approx(tolerance)
                .into_iter()
                .map(|(_, point_surface)| {
                    let point_global = surface
                        .geometry()
                        .point_from_surface_coords(point_surface);
                    ApproxPoint::new(point_surface, point_global)
                })
                .collect::<Vec<_>>();
        assert_eq!(approx.points, expected_approx);
    }
}
