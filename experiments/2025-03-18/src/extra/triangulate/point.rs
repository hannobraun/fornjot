use fj_math::Point;

use crate::geometry::SurfaceGeometry;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TriangulationPoint {
    pub point_surface: Point<2>,
    pub point_global: Point<3>,
}

impl TriangulationPoint {
    pub fn from_surface_point(
        point_surface: Point<2>,
        surface: &dyn SurfaceGeometry,
    ) -> Self {
        let point_global = surface.point_from_local(point_surface);

        Self {
            point_surface,
            point_global,
        }
    }
}

impl spade::HasPosition for TriangulationPoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.point_surface.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
