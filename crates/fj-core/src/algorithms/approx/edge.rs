//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left off, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to deal with duplicate vertices.

use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, GlobalPath, SurfacePath},
    objects::{GlobalEdge, HalfEdge, Surface, Vertex},
    storage::{Handle, HandleWrapper},
};

use super::{curve::CurveApproxSegment, Approx, ApproxPoint, Tolerance};

impl Approx for (&HalfEdge, &Surface) {
    type Approximation = HalfEdgeApprox;
    type Cache = EdgeCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (half_edge, surface) = self;

        let position_surface = half_edge.start_position();
        let position_global = match cache.get_position(half_edge.start_vertex())
        {
            Some(position) => position,
            None => {
                let position_global = surface
                    .geometry()
                    .point_from_surface_coords(position_surface);
                cache.insert_position(half_edge.start_vertex(), position_global)
            }
        };

        let first = ApproxPoint::new(position_surface, position_global);

        let points = {
            // We cache approximated `HalfEdge`s using the `GlobalEdge`s they
            // reference as the key. That bakes in the undesirable assumption
            // that all coincident `HalfEdge`s are also congruent. Let me
            // explain.
            //
            // When two `HalfEdge`s are coincident, we need to make sure their
            // approximations are identical where they overlap. Otherwise, we'll
            // get an invalid triangle mesh in the end. Hence, we cache
            // approximations.
            //
            // Caching works like this: We check whether there already is a
            // cache entry for the `GlobalEdge`. If there isn't, we create the
            // 3D approximation from the 2D `HalfEdge`. Next time we check for a
            // coincident `HalfEdge`, we'll find the cache and use that, getting
            // the exact same 3D approximation, instead of generating a slightly
            // different one from the different 2D `HalfEdge`.
            //
            // So what if we had two coincident `HalfEdge`s that aren't
            // congruent? Meaning, they overlap partially, but not fully. Then
            // obviously, they wouldn't refer to the same `GlobalEdge`, because
            // they are not the same edge, in global coordinates. And since the
            // `GlobalEdge` is the key in our cache, those `HalfEdge`s would not
            // share an approximation where they overlap, leading to exactly the
            // problems that the cache is supposed to avoid.
            //
            // As of this writing, it is a documented (but not validated)
            // limitation, that coincident `HalfEdge`s must always be congruent.
            // However, we're going to need to lift this limitation going
            // forward, as it is, well, too limiting. This means things here
            // will need to change.
            //
            // What we need here, is more intelligent caching based on `Curve`
            // and the edge boundaries, instead of `GlobalEdge`. The cache needs
            // to be able to deliver partial results for a given boundary, then
            // generating (and caching) the rest of it on the fly.
            let cached_approx = cache.get_edge(
                half_edge.global_form().clone(),
                half_edge.boundary(),
            );
            let approx = match cached_approx {
                Some(approx) => approx,
                None => {
                    let approx = approx_edge(
                        &half_edge.path(),
                        surface,
                        half_edge.boundary(),
                        tolerance,
                    );
                    cache.insert_edge(
                        half_edge.global_form().clone(),
                        half_edge.boundary(),
                        approx,
                    )
                }
            };

            approx
                .points
                .into_iter()
                .map(|point| {
                    let point_surface = half_edge
                        .path()
                        .point_from_path_coords(point.local_form);

                    ApproxPoint::new(point_surface, point.global_form)
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
    path: &SurfacePath,
    surface: &Surface,
    boundary: CurveBoundary<Point<1>>,
    tolerance: impl Into<Tolerance>,
) -> CurveApproxSegment {
    // There are different cases of varying complexity. Circles are the hard
    // part here, as they need to be approximated, while lines don't need to be.
    //
    // This will probably all be unified eventually, as `SurfacePath` and
    // `GlobalPath` grow APIs that are better suited to implementing this code
    // in a more abstract way.
    let points = match (path, surface.geometry().u) {
        (SurfacePath::Circle(_), GlobalPath::Circle(_)) => {
            todo!(
                "Approximating a circle on a curved surface not supported yet."
            )
        }
        (SurfacePath::Circle(_), GlobalPath::Line(_)) => {
            (path, boundary)
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
                CurveBoundary::from(boundary.inner.map(|point_curve| {
                    [path.point_from_path_coords(point_curve).u]
                }));

            let approx_u = (surface.geometry().u, range_u)
                .approx_with_cache(tolerance, &mut ());

            let mut points = Vec::new();
            for (u, _) in approx_u {
                let t = (u.t - line.origin().u) / line.direction().u;
                let point_surface = path.point_from_path_coords([t]);
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
    CurveApproxSegment { boundary, points }
}

/// A cache for results of an approximation
#[derive(Default)]
pub struct EdgeCache {
    edge_approx: BTreeMap<
        (HandleWrapper<GlobalEdge>, CurveBoundary<Point<1>>),
        CurveApproxSegment,
    >,
    vertex_approx: BTreeMap<HandleWrapper<Vertex>, Point<3>>,
}

impl EdgeCache {
    /// Create an empty cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Access the approximation for the given edge, if available
    fn get_edge(
        &self,
        handle: Handle<GlobalEdge>,
        boundary: CurveBoundary<Point<1>>,
    ) -> Option<CurveApproxSegment> {
        if let Some(approx) =
            self.edge_approx.get(&(handle.clone().into(), boundary))
        {
            return Some(approx.clone());
        }
        if let Some(approx) =
            self.edge_approx.get(&(handle.into(), boundary.reverse()))
        {
            // If we have a cache entry for the reverse boundary, we need to use
            // that too!
            return Some(approx.clone().reverse());
        }

        None
    }

    /// Insert the approximation of an edge
    fn insert_edge(
        &mut self,
        handle: Handle<GlobalEdge>,
        boundary: CurveBoundary<Point<1>>,
        approx: CurveApproxSegment,
    ) -> CurveApproxSegment {
        self.edge_approx
            .insert((handle.into(), boundary), approx.clone())
            .unwrap_or(approx)
    }

    fn get_position(&self, handle: &Handle<Vertex>) -> Option<Point<3>> {
        self.vertex_approx.get(&handle.clone().into()).cloned()
    }

    fn insert_position(
        &mut self,
        handle: &Handle<Vertex>,
        position: Point<3>,
    ) -> Point<3> {
        self.vertex_approx
            .insert(handle.clone().into(), position)
            .unwrap_or(position)
    }
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
        let half_edge =
            HalfEdge::line_segment([[1., 1.], [2., 1.]], None, &mut services);

        let tolerance = 1.;
        let approx = (&half_edge, surface.deref()).approx(tolerance);

        assert_eq!(approx.points, Vec::new());
    }

    #[test]
    fn approx_line_on_curved_surface_but_not_along_curve() {
        let mut services = Services::new();

        let surface = Surface::new(SurfaceGeometry {
            u: GlobalPath::circle_from_radius(1.),
            v: [0., 0., 1.].into(),
        });
        let half_edge =
            HalfEdge::line_segment([[1., 1.], [2., 1.]], None, &mut services);

        let tolerance = 1.;
        let approx = (&half_edge, &surface).approx(tolerance);

        assert_eq!(approx.points, Vec::new());
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
        let half_edge = HalfEdge::line_segment(
            [[0., 1.], [TAU, 1.]],
            Some(boundary.inner),
            &mut services,
        );

        let tolerance = 1.;
        let approx = (&half_edge, &surface).approx(tolerance);

        let expected_approx = (path, boundary)
            .approx(tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface =
                    half_edge.path().point_from_path_coords(point_local);
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
        let half_edge = HalfEdge::circle([0., 0.], 1., &mut services);

        let tolerance = 1.;
        let approx = (&half_edge, surface.deref()).approx(tolerance);

        let expected_approx =
            (&half_edge.path(), CurveBoundary::from([[0.], [TAU]]))
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
