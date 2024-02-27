use fj_math::{Point, Scalar, Vector};

use crate::{
    geometry::{GlobalPath, SurfaceGeometry},
    objects::Surface,
    operations::insert::Insert,
    storage::Handle,
    Core,
};

/// Build a [`Surface`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildSurface {
    /// Build a plane from the provided points
    fn plane_from_points(
        points: [impl Into<Point<3>>; 3],
        core: &mut Core,
    ) -> (Handle<Surface>, [Point<2>; 3]) {
        let [a, b, c] = points.map(Into::into);

        let (u, u_line) = GlobalPath::line_from_points([a, b]);
        let v = c - a;

        let surface = Surface::surface_from_uv(u, v).insert(core);

        let points_surface = {
            let [a, b] =
                u_line.map(|point| Point::from([point.t, Scalar::ZERO]));
            let c = Point::from([a.u, Scalar::ONE]);

            [a, b, c]
        };

        (surface, points_surface)
    }

    /// Build a plane from the provided `u` and `v`
    fn surface_from_uv(
        u: impl Into<GlobalPath>,
        v: impl Into<Vector<3>>,
    ) -> Surface {
        let geometry = SurfaceGeometry {
            u: u.into(),
            v: v.into(),
        };
        Surface::new(geometry)
    }
}

impl BuildSurface for Surface {}
