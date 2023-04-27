use std::ops::Deref;

use fj_interop::{debug::DebugInfo, ext::ArrayExt, mesh::Color};
use fj_kernel::{
    algorithms::reverse::Reverse,
    objects::{Region, Sketch},
    operations::Insert,
    services::Services,
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Difference2d {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        services: &mut Services,
        debug_info: &mut DebugInfo,
    ) -> Self::Brep {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut regions = Vec::new();

        let mut exteriors = Vec::new();
        let mut interiors = Vec::new();

        let [a, b] = self
            .shapes()
            .each_ref_ext()
            .map(|shape| shape.compute_brep(services, debug_info));

        if a.regions().into_iter().next().is_some() {
            // If there's at least one region to subtract from, we can proceed.

            for region in a.regions() {
                exteriors.push(region.exterior().clone());
                for cycle in region.interiors() {
                    interiors.push(cycle.clone().reverse(services));
                }
            }

            for region in b.regions() {
                interiors.push(region.exterior().clone().reverse(services));
            }

            // Faces only support one exterior, while the code here comes from
            // the time when a face could have multiple exteriors. This was only
            // a special case, i.e. faces that connected to themselves, and I
            // have my doubts that this code was ever correct in the first
            // place.
            //
            // Anyway, the following should make sure that at least any problems
            // this code causes become obvious. I don't know if this can ever
            // trigger, but better safe than sorry.
            let exterior = exteriors
                .pop()
                .expect("Can't construct face without an exterior");
            assert!(
                exteriors.is_empty(),
                "Can't construct face with multiple exteriors"
            );

            let region =
                Region::new(exterior, interiors, Some(Color(self.color())));
            regions.push(region.insert(services));
        }

        let difference = Sketch::new(regions).insert(services);
        difference.deref().clone()
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.shapes()[0].bounding_volume()
    }
}
