use fj_math::Point;

use crate::{
    geometry::{curve::GlobalPath, surface::SurfaceGeometry},
    objects::Surface,
};

/// Build a [`Surface`]
pub trait BuildSurface {
    /// Build a plane from the provided points
    fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Surface {
        let [a, b, c] = points.map(Into::into);

        let u = GlobalPath::line_from_points([a, b]).0;
        let v = c - a;

        let geometry = SurfaceGeometry { u, v };
        Surface::new(geometry)
    }
}

impl BuildSurface for Surface {}
