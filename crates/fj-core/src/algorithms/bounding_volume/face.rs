use std::ops::Deref;

use fj_interop::Tolerance;
use fj_math::{Aabb, Vector};

use crate::{
    geometry::{
        Geometry, repr::tri_mesh::convert_point_surface_to_global,
        traits::GenTriMesh,
    },
    topology::Face,
};

impl super::BoundingVolume<3> for &Face {
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<3>> {
        // A bounding volume must include the body it bounds, but does not need
        // to match it precisely. So it's okay, if it's a bit larger.
        //
        // Let's just choose a reasonable tolerance value here, that we can use
        // to enlarge any AABBs we use in this method as necessary.
        let tolerance = Tolerance::from_scalar(0.001)
            .expect("Tolerance provided is larger than zero");

        (self.region().exterior().deref(), self.surface())
            .aabb(geometry)
            .map(|aabb2| {
                let surface =
                    &geometry.of_surface_2(self.surface()).unwrap().generator;
                let tri_mesh =
                    surface.generate_tri_mesh(aabb2, tolerance, geometry);
                let tri_mesh = tri_mesh.into_iter().map(|point| {
                    convert_point_surface_to_global(
                        surface, point, tolerance, geometry,
                    )
                });

                let mut aabb3 = Aabb::<3>::from_points(tri_mesh);

                let offset = Vector::from([tolerance.inner(); 3]);

                aabb3.min -= offset;
                aabb3.max += offset;

                aabb3
            })
    }
}
