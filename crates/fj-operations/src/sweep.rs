use std::ops::Deref;

use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::sweep::Sweep,
    insert::Insert,
    objects::{Objects, Solid},
    validate::ValidationError,
};
use fj_math::{Aabb, Vector};

use super::Shape;

impl Shape for fj::Sweep {
    type Brep = Solid;

    fn compute_brep(
        &self,
        objects: &mut Objects,
        debug_info: &mut DebugInfo,
    ) -> Result<Self::Brep, ValidationError> {
        let sketch = self.shape().compute_brep(objects, debug_info)?;
        let sketch = sketch.insert(objects)?;

        let path = Vector::from(self.path());

        let solid = sketch.sweep(path, objects)?;
        Ok(solid.deref().clone())
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
