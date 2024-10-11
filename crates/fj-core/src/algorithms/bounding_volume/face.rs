use std::ops::Deref;

use fj_math::{Aabb, Vector};

use crate::{
    geometry::{
        surfaces::SweptCurve, util::tri_mesh::convert_point_surface_to_global,
        Geometry, Path, Tolerance,
    },
    topology::Face,
};

impl super::BoundingVolume<3> for &Face {
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<3>> {
        (self.region().exterior().deref(), self.surface())
            .aabb(geometry)
            .map(|aabb2| {
                let surface = geometry.of_surface(self.surface());

                let SweptCurve { u, v } = surface;
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
                        let tolerance = Tolerance::from_scalar(0.001)
                            .expect("Tolerance provided is larger than zero");
                        let offset = Vector::from([tolerance.inner(); 3]);

                        Aabb {
                            min: convert_point_surface_to_global(
                                surface, aabb2.min, tolerance,
                            ) - offset,
                            max: convert_point_surface_to_global(
                                surface, aabb2.max, tolerance,
                            ) + offset,
                        }
                    }
                }
            })
    }
}
