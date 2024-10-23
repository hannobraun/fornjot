use fj_math::{Circle, Line, Vector};

use crate::{
    geometry::{
        surfaces::SweptCurve,
        util::tri_mesh::{
            convert_point_surface_to_global, convert_vector_surface_to_global,
        },
        Path,
    },
    operations::build::BuildSurface,
    storage::Handle,
    topology::Surface,
    Core,
};

/// # Sweep a [`Path`]
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
        surface: &SweptCurve,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Handle<Surface>;
}

impl SweepSurfacePath for Path<2> {
    fn sweep_surface_path(
        &self,
        surface: &SweptCurve,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Handle<Surface> {
        let SweptCurve { u, .. } = surface;
        match u {
            Path::Circle(_) => {
                // Sweeping a `Curve` creates a `Surface`. The u-axis of that
                // `Surface` is a `Path<3>`, which we are computing below. That
                // computation might or might not work with an arbitrary
                // surface. Probably not, but I'm not sure.
                //
                // What definitely won't work, is computing the bottom edge of
                // the sweep. The edge sweeping code currently assumes that the
                // bottom edge is a line (which is true when sweeping from a
                // flat surface). But if the surface we're sweeping from is
                // curved, there's simply no way to represent the curve of the
                // resulting bottom edge.
                todo!(
                    "Sweeping a curve that is defined on a curved surface is \
                    not supported yet."
                )
            }
            Path::Line(_) => {
                // We're sweeping from a curve on a flat surface, which is
                // supported. Carry on.
            }
        }

        let u = match self {
            Path::Circle(circle) => {
                let center = convert_point_surface_to_global(
                    surface,
                    circle.center(),
                    core.tolerance(),
                    &core.layers.geometry,
                );
                let a = convert_vector_surface_to_global(
                    surface,
                    circle.a(),
                    core.tolerance(),
                    &core.layers.geometry,
                );
                let b = convert_vector_surface_to_global(
                    surface,
                    circle.b(),
                    core.tolerance(),
                    &core.layers.geometry,
                );

                let circle = Circle::new(center, a, b);

                Path::Circle(circle)
            }
            Path::Line(line) => {
                let origin = convert_point_surface_to_global(
                    surface,
                    line.origin(),
                    core.tolerance(),
                    &core.layers.geometry,
                );
                let direction = convert_vector_surface_to_global(
                    surface,
                    line.direction(),
                    core.tolerance(),
                    &core.layers.geometry,
                );

                let line = Line::from_origin_and_direction(origin, direction);

                Path::Line(line)
            }
        };

        Surface::from_uv(u, path, core)
    }
}
