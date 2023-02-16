use fj_math::{Circle, Line, Vector};

use crate::{
    builder::SurfaceBuilder,
    geometry::path::{GlobalPath, SurfacePath},
    insert::Insert,
    objects::{Curve, Objects, Surface},
    partial::{PartialObject, PartialSurface},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for (Handle<Curve>, &Surface) {
    type Swept = Handle<Surface>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        _: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let (curve, surface) = self;

        match surface.geometry().u {
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

        let u = match curve.path() {
            SurfacePath::Circle(circle) => {
                let center = surface
                    .geometry()
                    .point_from_surface_coords(circle.center());
                let a =
                    surface.geometry().vector_from_surface_coords(circle.a());
                let b =
                    surface.geometry().vector_from_surface_coords(circle.b());

                let circle = Circle::new(center, a, b);

                GlobalPath::Circle(circle)
            }
            SurfacePath::Line(line) => {
                let origin =
                    surface.geometry().point_from_surface_coords(line.origin());
                let direction = surface
                    .geometry()
                    .vector_from_surface_coords(line.direction());

                let line = Line::from_origin_and_direction(origin, direction);

                GlobalPath::Line(line)
            }
        };

        PartialSurface::from_axes(u, path)
            .build(objects)
            .insert(objects)
    }
}
