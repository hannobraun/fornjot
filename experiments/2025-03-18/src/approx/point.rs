use fj_math::Point;

use crate::geometry::SurfaceGeometry;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApproxPoint<const D: usize> {
    pub point_surface: Point<D>,
    pub global: Point<3>,
}

impl ApproxPoint<2> {
    pub fn from_surface_point(
        point_surface: Point<2>,
        surface: &dyn SurfaceGeometry,
    ) -> Self {
        let point_global = surface.point_from_local(point_surface);

        Self {
            point_surface,
            global: point_global,
        }
    }
}

impl spade::HasPosition for ApproxPoint<2> {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.point_surface.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
