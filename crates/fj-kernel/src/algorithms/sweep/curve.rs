use fj_math::{Circle, Line, Vector};

use crate::{
    geometry::path::{GlobalPath, SurfacePath},
    objects::{Curve, Objects, Surface},
    storage::Handle,
    validate::ValidationError,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Curve> {
    type Swept = Handle<Surface>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        _: &mut SweepCache,
        objects: &Objects,
    ) -> Result<Self::Swept, ValidationError> {
        match self.surface().u() {
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

        let u = match self.path() {
            SurfacePath::Circle(circle) => {
                let center =
                    self.surface().point_from_surface_coords(circle.center());
                let a = self.surface().vector_from_surface_coords(circle.a());
                let b = self.surface().vector_from_surface_coords(circle.b());

                let circle = Circle::new(center, a, b);

                GlobalPath::Circle(circle)
            }
            SurfacePath::Line(line) => {
                let origin =
                    self.surface().point_from_surface_coords(line.origin());
                let direction =
                    self.surface().vector_from_surface_coords(line.direction());

                let line = Line::from_origin_and_direction(origin, direction);

                GlobalPath::Line(line)
            }
        };

        Ok(objects.surfaces.insert(Surface::new(u, path))?)
    }
}
