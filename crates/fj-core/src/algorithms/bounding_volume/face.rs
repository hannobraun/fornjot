use std::ops::Deref;

use fj_math::{Aabb, Vector};

use crate::{
    geometry::{Geometry, Path, SurfaceGeom, Tolerance},
    topology::Face,
};

impl super::BoundingVolume<3> for &Face {
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<3>> {
        (self.region().exterior().deref(), self.surface())
            .aabb(geometry)
            .map(|aabb2| {
                let surface = geometry.of_surface(self.surface());

                let SurfaceGeom { u, v } = surface;
                match u {
                    Path::Circle(circle) => {
                        // This is not the most precise way to calculate the
                        // AABB, doing it for the whole circle, but it should
                        // do.

                        let aabb_bottom = circle.aabb();
                        let aabb_top = Aabb {
                            min: aabb_bottom.min + *v,
                            max: aabb_bottom.max + *v,
                        };

                        aabb_bottom.merged(&aabb_top)
                    }
                    Path::Line(_) => {
                        // A bounding volume must include the body it bounds,
                        // but does not need to match it precisely. So it's
                        // okay, if it's a bit larger.
                        //
                        // Let's just choose a reasonable tolerance value here,
                        // then make sure we enlarge the AABB accordingly, to
                        // make sure it fits.
                        let tolerance_f64 = 0.001;
                        let tolerance = Tolerance::from_scalar(tolerance_f64)
                            .expect("Tolerance provided is larger than zero");
                        let offset = Vector::from([tolerance_f64; 3]);

                        Aabb {
                            min: surface.point_from_surface_coords(
                                aabb2.min, tolerance,
                            ) - offset,
                            max: surface.point_from_surface_coords(
                                aabb2.max, tolerance,
                            ) + offset,
                        }
                    }
                }
            })
    }
}
