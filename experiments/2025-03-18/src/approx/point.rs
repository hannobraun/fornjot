use std::fmt;

use fj_math::Point;

use crate::geometry::{CurveGeometry, SurfaceGeometry};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApproxPoint<const D: usize> {
    pub local: Point<D>,
    pub global: Point<3>,
}

impl ApproxPoint<1> {
    pub fn from_curve_point(
        origin: Point<3>,
        point_curve: Point<1>,
        curve: &dyn CurveGeometry,
    ) -> Self {
        let vector_global = curve.vector_from_local_point(point_curve);
        let point_global = origin + vector_global;

        Self {
            local: point_curve,
            global: point_global,
        }
    }
}

impl ApproxPoint<2> {
    pub fn from_surface_point(
        point_surface: Point<2>,
        surface: &dyn SurfaceGeometry,
    ) -> Self {
        let point_global = surface.point_from_local(point_surface);

        Self {
            local: point_surface,
            global: point_global,
        }
    }
}

impl<const D: usize> fmt::Display for ApproxPoint<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{local:.3?} / {global:.3?}",
            local = self.local,
            global = self.global,
        )
    }
}

impl spade::HasPosition for ApproxPoint<2> {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.local.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
