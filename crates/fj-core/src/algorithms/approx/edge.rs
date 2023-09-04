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
    objects::{Curve, Edge, Surface, Vertex},
    storage::{Handle, HandleWrapper},
};

use super::{curve::CurveApproxSegment, Approx, ApproxPoint, Tolerance};

impl Approx for (&Edge, &Surface) {
    type Approximation = EdgeApprox;
    type Cache = EdgeCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (edge, surface) = self;

        let start_position_surface = edge.start_position();
        let start_position =
            match cache.get_start_position_approx(edge.start_vertex()) {
                Some(position) => position,
                None => {
                    let position_global = surface
                        .geometry()
                        .point_from_surface_coords(start_position_surface);
                    cache.insert_start_position_approx(
                        edge.start_vertex(),
                        position_global,
                    )
                }
            };

        let first = ApproxPoint::new(start_position_surface, start_position);

        let rest = {
            // We cache approximated `Edge`s using the `Curve`s they reference
            // and their boundary on that curve as the key. That bakes in the
            // undesirable assumption that all coincident `Edge`s are also
            // congruent. Let me explain.
            //
            // When two `Edge`s are coincident, we need to make sure their
            // approximations are identical where they overlap. Otherwise, we'll
            // get an invalid triangle mesh in the end. Hence, we cache
            // approximations.
            //
            // Caching works like this: We check whether there already is a
            // cache entry for the curve/boundary. If there isn't, we create the
            // 3D approximation from the 2D `Edge`. Next time we check for a
            // coincident `Edge`, we'll find the cache and use that, getting
            // the exact same 3D approximation, instead of generating a slightly
            // different one from the different 2D `Edge`.
            //
            // So what if we had two coincident `fEdge`s that aren't congruent?
            // Meaning, they overlap partially, but not fully. Then obviously,
            // they wouldn't refer to the same combination of curve and
            // boundary. And since those are the key in our cache, those `Edge`s
            // would not share an approximation where they overlap, leading to
            // exactly the problems that the cache is supposed to prevent.
            //
            // As of this writing, it is a documented (but not validated)
            // limitation, that coincident `Edge`s must always be congruent.
            // However, we're going to need to lift this limitation going
            // forward, as it is, well, too limiting. This means things here
            // will need to change.
            //
            // What we need here, is more intelligent caching, that can reuse
            // already cached curve approximations where the boundaries overlap,
            // even if those boundaries aren't identical. The cache needs to be
            // able to deliver partial results for a given boundary, then
            // generating (and caching) the rest of it on the fly.
            let cached_approx =
                cache.get_edge(edge.curve().clone(), edge.boundary());
            let approx = match cached_approx {
                Some(approx) => approx,
                None => {
                    let approx = approx_edge(
                        &edge.path(),
                        surface,
                        edge.boundary(),
                        tolerance,
                    );
                    cache.insert_edge(
                        edge.curve().clone(),
                        edge.boundary(),
                        approx,
                    )
                }
            };

            approx
                .points
                .into_iter()
                .map(|point| {
                    let point_surface =
                        edge.path().point_from_path_coords(point.local_form);

                    ApproxPoint::new(point_surface, point.global_form)
                })
                .collect()
        };

        EdgeApprox { first, rest }
    }
}

/// An approximation of an [`Edge`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EdgeApprox {
    /// The point that approximates the first vertex of the edge
    first: ApproxPoint<2>,

    /// The approximation of the rest of the edge
    rest: Vec<ApproxPoint<2>>,
}

impl EdgeApprox {
    /// Compute the points that approximate the edge
    pub fn points(&self) -> Vec<ApproxPoint<2>> {
        let mut points = Vec::new();

        points.push(self.first.clone());
        points.extend(self.rest.iter().cloned());

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
    vertex_approx: BTreeMap<HandleWrapper<Vertex>, Point<3>>,
    edge_approx: BTreeMap<
        (HandleWrapper<Curve>, CurveBoundary<Point<1>>),
        CurveApproxSegment,
    >,
}

impl EdgeCache {
    /// Create an empty cache
    pub fn new() -> Self {
        Self::default()
    }

    fn get_start_position_approx(
        &self,
        handle: &Handle<Vertex>,
    ) -> Option<Point<3>> {
        self.vertex_approx.get(&handle.clone().into()).cloned()
    }

    fn insert_start_position_approx(
        &mut self,
        handle: &Handle<Vertex>,
        position: Point<3>,
    ) -> Point<3> {
        self.vertex_approx
            .insert(handle.clone().into(), position)
            .unwrap_or(position)
    }

    /// Access the approximation for the given edge, if available
    fn get_edge(
        &self,
        handle: Handle<Curve>,
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
        handle: Handle<Curve>,
        boundary: CurveBoundary<Point<1>>,
        approx: CurveApproxSegment,
    ) -> CurveApproxSegment {
        self.edge_approx
            .insert((handle.into(), boundary), approx.clone())
            .unwrap_or(approx)
    }
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::TAU, ops::Deref};

    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::approx::{Approx, ApproxPoint},
        geometry::{CurveBoundary, GlobalPath, SurfaceGeometry},
        objects::{Edge, Surface},
        operations::BuildEdge,
        services::Services,
    };

    #[test]
    fn approx_line_on_flat_surface() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xz_plane();
        let edge =
            Edge::line_segment([[1., 1.], [2., 1.]], None, &mut services);

        let tolerance = 1.;
        let approx = (&edge, surface.deref()).approx(tolerance);

        assert_eq!(approx.rest, Vec::new());
    }

    #[test]
    fn approx_line_on_curved_surface_but_not_along_curve() {
        let mut services = Services::new();

        let surface = Surface::new(SurfaceGeometry {
            u: GlobalPath::circle_from_radius(1.),
            v: [0., 0., 1.].into(),
        });
        let edge =
            Edge::line_segment([[1., 1.], [2., 1.]], None, &mut services);

        let tolerance = 1.;
        let approx = (&edge, &surface).approx(tolerance);

        assert_eq!(approx.rest, Vec::new());
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
        let edge = Edge::line_segment(
            [[0., 1.], [TAU, 1.]],
            Some(boundary.inner),
            &mut services,
        );

        let tolerance = 1.;
        let approx = (&edge, &surface).approx(tolerance);

        let expected_approx = (path, boundary)
            .approx(tolerance)
            .into_iter()
            .map(|(point_local, _)| {
                let point_surface =
                    edge.path().point_from_path_coords(point_local);
                let point_global =
                    surface.geometry().point_from_surface_coords(point_surface);
                ApproxPoint::new(point_surface, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx.rest, expected_approx);
    }

    #[test]
    fn approx_circle_on_flat_surface() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xz_plane();
        let edge = Edge::circle([0., 0.], 1., &mut services);

        let tolerance = 1.;
        let approx = (&edge, surface.deref()).approx(tolerance);

        let expected_approx =
            (&edge.path(), CurveBoundary::from([[0.], [TAU]]))
                .approx(tolerance)
                .into_iter()
                .map(|(_, point_surface)| {
                    let point_global = surface
                        .geometry()
                        .point_from_surface_coords(point_surface);
                    ApproxPoint::new(point_surface, point_global)
                })
                .collect::<Vec<_>>();
        assert_eq!(approx.rest, expected_approx);
    }
}
