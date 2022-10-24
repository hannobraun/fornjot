use std::ops::Deref;

use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::sweep::Sweep,
    objects::{Objects, Solid},
    validate::{Validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Vector};

use super::Shape;

impl Shape for fj::Sweep {
    type Brep = Solid;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        objects: &Objects,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let sketch = self.shape().compute_brep(config, objects, debug_info)?;
        let sketch = objects.sketches.insert(sketch.into_inner());

        let path = Vector::from(self.path());

        let solid = sketch.sweep(path, objects);
        solid.deref().clone().validate_with_config(config)
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
