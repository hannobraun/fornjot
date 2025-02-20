use fj_math::{Point, Scalar, Vector};

use crate::{
    Core,
    geometry::{
        Path, SurfaceGeom, repr::tri_mesh::TriMesh, surfaces::SweptCurve,
    },
    operations::insert::Insert,
    storage::Handle,
    topology::Surface,
};

/// Build a [`Surface`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildSurface {
    /// Build a surface from the provided geometry
    fn from_geometry(
        generator: SweptCurve,
        core: &mut Core,
    ) -> Handle<Surface> {
        let surface = Surface::new().insert(core);

        core.layers
            .geometry
            .define_surface(surface.clone(), generator);
        core.layers.geometry.define_surface_2(
            surface.clone(),
            SurfaceGeom {
                generator: Box::new(generator),
                geometry: TriMesh::empty(),
            },
        );

        surface
    }

    /// Build a surface from the provided `u` and `v`
    fn from_uv(
        u: impl Into<Path<3>>,
        v: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Handle<Surface> {
        Self::from_geometry(
            SweptCurve {
                u: u.into(),
                v: v.into(),
            },
            core,
        )
    }

    /// Build a plane from the provided points
    fn plane_from_points(
        points: [impl Into<Point<3>>; 3],
        core: &mut Core,
    ) -> (Handle<Surface>, [Point<2>; 3]) {
        let [a, b, c] = points.map(Into::into);

        let (u, u_line) = Path::line_from_points([a, b]);
        let v = c - a;

        let surface = Surface::from_uv(u, v, core);

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
