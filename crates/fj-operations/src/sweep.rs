use std::ops::Deref;

use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::sweep::Sweep,
    insert::Insert,
    objects::{Objects, Solid},
    services::Service,
};
use fj_math::{Aabb, Vector};

use super::Shape;

impl Shape for fj::Sweep {
    type Brep = Solid;

    fn compute_brep(
        &self,
        objects: &mut Service<Objects>,
        debug_info: &mut DebugInfo,
    ) -> Self::Brep {
        let sketch = self.shape().compute_brep(objects, debug_info);
        let sketch = sketch.insert(objects);

        let path = Vector::from(self.path());

        let solid = sketch.sweep(path, objects);
        solid.deref().clone()
    }

    fn bounding_volume(&self) -> Aabb<3> {
        self.shape()
            .bounding_volume()
            .merged(&Aabb::<3>::from_points(
                self.shape()
                    .bounding_volume()
                    .vertices()
                    .map(|v| v + self.path()),
            ))
    }
}
