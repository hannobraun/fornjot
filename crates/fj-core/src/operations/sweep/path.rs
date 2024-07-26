use fj_math::{Circle, Line, Vector};

use crate::{
    geometry::{GlobalPath, SurfaceGeom, SurfacePath},
    operations::build::BuildSurface,
    storage::Handle,
    topology::Surface,
    Core,
};

/// # Sweep a [`SurfacePath`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepSurfacePath {
    /// # Sweep the surface path
    ///
    /// Requires a reference to the surface that the path is defined on.
    ///
    ///
    /// ## Implementation Note
    ///
    /// Sweeping a `SurfacePath` that is defined on a curved surface is
    /// currently not supported:
    /// <https://github.com/hannobraun/fornjot/issues/1112>
    fn sweep_surface_path(
        &self,
        surface: &SurfaceGeom,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Handle<Surface>;
}

impl SweepSurfacePath for SurfacePath {
    fn sweep_surface_path(
        &self,
        surface: &SurfaceGeom,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Handle<Surface> {
        let SurfaceGeom { u, .. } = surface;
        match u {
            GlobalPath::Circle(_) => {
                // Sweeping a `Curve` creates a `Surface`. The u-axis of that
                // `Surface` is a `GlobalPath`, which we are computing below.
                // That computation might or might not work with an arbitrary
                // surface. Probably not, but I'm not sure.
                //
                // What definitely won't work, is computing the bottom edge of
                // the sweep. The edge sweeping code currently assumes that the
                // bottom edge is a line (which is true when sweeping from a
                // flat surface). But is the surface we're sweeping from is
                // curved, there's simply no way to represent the curve of the
                // resulting bottom edge.
                todo!(
                    "Sweeping a curve that is defined on a curved surface is \
                    not supported yet."
                )
            }
            GlobalPath::Line(_) => {
                // We're sweeping from a curve on a flat surface, which is
                // supported. Carry on.
            }
        }

        let u = match self {
            SurfacePath::Circle(circle) => {
                let center = surface.point_from_surface_coords(circle.center());
                let a = surface.vector_from_surface_coords(circle.a());
                let b = surface.vector_from_surface_coords(circle.b());

                let circle = Circle::new(center, a, b);

                GlobalPath::Circle(circle)
            }
            SurfacePath::Line(line) => {
                let origin = surface.point_from_surface_coords(line.origin());
                let direction =
                    surface.vector_from_surface_coords(line.direction());

                let line = Line::from_origin_and_direction(origin, direction);

                GlobalPath::Line(line)
            }
        };

        Surface::from_uv(u, path, core)
    }
}
