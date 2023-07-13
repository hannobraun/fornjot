use fj_math::{Point, Scalar};

use crate::{
    geometry::{GlobalPath, SurfaceGeometry},
    objects::Surface,
};

/// Build a [`Surface`]
pub trait BuildSurface {
    /// Build a plane from the provided points
    fn plane_from_points(
        points: [impl Into<Point<3>>; 3],
    ) -> (Surface, [Point<2>; 3]) {
        let [a, b, c] = points.map(Into::into);

        let (u, u_line) = GlobalPath::line_from_points([a, b]);
        let v = c - a;

        let geometry = SurfaceGeometry { u, v };
        let surface = Surface::new(geometry);

        let points_surface = {
            let [a, b] =
                u_line.map(|point| Point::from([point.t, Scalar::ZERO]));
            let c = Point::from([a.u, Scalar::ONE]);

            [a, b, c]
        };

        (surface, points_surface)
    }
}

impl BuildSurface for Surface {}
