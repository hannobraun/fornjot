use fj_math::Aabb;

use crate::{geometry::curve::GlobalPath, objects::Face};

impl super::BoundingVolume<3> for Face {
    fn aabb(&self) -> Option<Aabb<3>> {
        self.exterior().aabb().map(|aabb2| {
            let surface = self.surface().geometry();

            match surface.u {
                GlobalPath::Circle(_) => {
                    // I don't currently have an example model to test this
                    // with. This should change soon, and then this will panic
                    // and can be addressed.
                    todo!("Computing AABB of curved face is not supported yet")
                }
                GlobalPath::Line(_) => Aabb {
                    min: surface.point_from_surface_coords(aabb2.min),
                    max: surface.point_from_surface_coords(aabb2.max),
                },
            }
        })
    }
}
