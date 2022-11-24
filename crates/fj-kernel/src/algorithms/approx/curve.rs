//! Curve approximation
//!
//! Since curves are infinite (even circles have an infinite coordinate space,
//! even though they connect to themselves in global coordinates), a range must
//! be provided to approximate them. The approximation then returns points
//! within that range.
//!
//! The boundaries of the range are not included in the approximation. This is
//! done, to give the caller (who knows the boundary anyway) more options on how
//! to further process the approximation.

use std::collections::BTreeMap;

use crate::{
    geometry::path::{GlobalPath, SurfacePath},
    objects::{Curve, GlobalCurve},
    storage::{Handle, ObjectId},
};

use super::{path::RangeOnPath, Approx, ApproxPoint, Tolerance};

impl Approx for (&Handle<Curve>, RangeOnPath) {
    type Approximation = CurveApprox;
    type Cache = CurveCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        let global_curve = curve.global_form().clone();
        let global_curve_approx = match cache.get(global_curve.clone(), range) {
            Some(approx) => approx,
            None => {
                let approx = approx_global_curve(curve, range, tolerance);
                cache.insert(global_curve, range, approx)
            }
        };

        CurveApprox::empty().with_points(
            global_curve_approx.points.into_iter().map(|point| {
                let point_surface =
                    curve.path().point_from_path_coords(point.local_form);

                ApproxPoint::new(point_surface, point.global_form)
                    .with_source((curve.clone(), point.local_form))
            }),
        )
    }
}

fn approx_global_curve(
    curve: &Curve,
    range: RangeOnPath,
    tolerance: impl Into<Tolerance>,
) -> GlobalCurveApprox {
    // There are different cases of varying complexity. Circles are the hard
    // part here, as they need to be approximated, while lines don't need to be.
    //
    // This will probably all be unified eventually, as `SurfacePath` and
    // `GlobalPath` grow APIs that are better suited to implementing this code
    // in a more abstract way.
    let points = match (curve.path(), curve.surface().geometry().u) {
        (SurfacePath::Circle(_), GlobalPath::Circle(_)) => {
            todo!(
                "Approximating a circle on a curved surface not supported yet."
            )
        }
        (SurfacePath::Circle(_), GlobalPath::Line(_)) => {
            (curve.path(), range)
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

                    let point_global = curve
                        .surface()
                        .geometry()
                        .point_from_surface_coords(point_surface);
                    (point_curve, point_global)
                })
                .collect()
        }
        (SurfacePath::Line(line), _) => {
            let range_u =
                RangeOnPath::from(range.boundary.map(|point_curve| {
                    [curve.path().point_from_path_coords(point_curve).u]
                }));

            let approx_u = (curve.surface().geometry().u, range_u)
                .approx_with_cache(tolerance, &mut ());

            let mut points = Vec::new();
            for (u, _) in approx_u {
                let t = (u.t - line.origin().u) / line.direction().u;
                let point_surface = curve.path().point_from_path_coords([t]);
                let point_global = curve
                    .surface()
                    .geometry()
                    .point_from_surface_coords(point_surface);
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
    GlobalCurveApprox { points }
}

/// An approximation of a [`Curve`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<2>>,
}

impl CurveApprox {
    /// Create an empty instance of `CurveApprox`
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// Add points to the approximation
    pub fn with_points(
        mut self,
        points: impl IntoIterator<Item = ApproxPoint<2>>,
    ) -> Self {
        self.points.extend(points);
        self
    }
}

/// A cache for results of an approximation
#[derive(Default)]
pub struct CurveCache {
    inner: BTreeMap<(ObjectId, RangeOnPath), GlobalCurveApprox>,
}

impl CurveCache {
    /// Create an empty cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert the approximation of a [`GlobalCurve`]
    pub fn insert(
        &mut self,
        handle: Handle<GlobalCurve>,
        range: RangeOnPath,
        approx: GlobalCurveApprox,
    ) -> GlobalCurveApprox {
        self.inner.insert((handle.id(), range), approx.clone());
        approx
    }

    /// Access the approximation for the given [`GlobalCurve`], if available
    pub fn get(
        &self,
        handle: Handle<GlobalCurve>,
        range: RangeOnPath,
    ) -> Option<GlobalCurveApprox> {
        self.inner.get(&(handle.id(), range)).cloned()
    }
}

/// An approximation of a [`GlobalCurve`]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalCurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<1>>,
}

#[cfg(test)]
mod tests {
    use std::f64::consts::TAU;

    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::approx::{path::RangeOnPath, Approx, ApproxPoint},
        builder::{CurveBuilder, SurfaceBuilder},
        geometry::path::GlobalPath,
        insert::Insert,
        objects::Objects,
        partial::{PartialCurve, PartialSurface},
    };

    use super::CurveApprox;

    #[test]
    fn approx_line_on_flat_surface() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let surface =
            PartialSurface::from_axes(GlobalPath::x_axis(), [0., 0., 1.])
                .build(&objects)?
                .insert(&objects)?;
        let mut curve = PartialCurve {
            surface: Some(surface),
            ..Default::default()
        };
        curve.update_as_line_from_points([[1., 1.], [2., 1.]]);
        let curve = curve.build(&mut objects)?.insert(&objects)?;
        let range = RangeOnPath::from([[0.], [1.]]);

        let approx = (&curve, range).approx(1.);

        assert_eq!(approx, CurveApprox::empty());
        Ok(())
    }

    #[test]
    fn approx_line_on_curved_surface_but_not_along_curve() -> anyhow::Result<()>
    {
        let mut objects = Objects::new();

        let surface = PartialSurface::from_axes(
            GlobalPath::circle_from_radius(1.),
            [0., 0., 1.],
        )
        .build(&objects)?
        .insert(&objects)?;
        let mut curve = PartialCurve {
            surface: Some(surface),
            ..Default::default()
        };
        curve.update_as_line_from_points([[1., 1.], [1., 2.]]);
        let curve = curve.build(&mut objects)?.insert(&objects)?;
        let range = RangeOnPath::from([[0.], [1.]]);

        let approx = (&curve, range).approx(1.);

        assert_eq!(approx, CurveApprox::empty());
        Ok(())
    }

    #[test]
    fn approx_line_on_curved_surface_along_curve() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let path = GlobalPath::circle_from_radius(1.);
        let surface = PartialSurface::from_axes(path, [0., 0., 1.])
            .build(&objects)?
            .insert(&objects)?;
        let mut curve = PartialCurve {
            surface: Some(surface.clone()),
            ..Default::default()
        };
        curve.update_as_line_from_points([[0., 1.], [1., 1.]]);
        let curve = curve.build(&mut objects)?.insert(&objects)?;

        let range = RangeOnPath::from([[0.], [TAU]]);
        let tolerance = 1.;

        let approx = (&curve, range).approx(tolerance);

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
        assert_eq!(approx.points, expected_approx);
        Ok(())
    }

    #[test]
    fn approx_circle_on_flat_surface() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let surface =
            PartialSurface::from_axes(GlobalPath::x_axis(), [0., 0., 1.])
                .build(&objects)?
                .insert(&objects)?;
        let mut curve = PartialCurve {
            surface: Some(surface),
            ..Default::default()
        };
        curve.update_as_circle_from_radius(1.);
        let curve = curve.build(&mut objects)?.insert(&objects)?;

        let range = RangeOnPath::from([[0.], [TAU]]);
        let tolerance = 1.;
        let approx = (&curve, range).approx(tolerance);

        let expected_approx = (curve.path(), range)
            .approx(tolerance)
            .into_iter()
            .map(|(_, point_surface)| {
                let point_global = curve
                    .surface()
                    .geometry()
                    .point_from_surface_coords(point_surface);
                ApproxPoint::new(point_surface, point_global)
            })
            .collect::<Vec<_>>();
        assert_eq!(approx.points, expected_approx);
        Ok(())
    }
}
