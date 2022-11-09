use fj_interop::debug::DebugInfo;
use fj_kernel::{
    objects::{FaceSet, Objects},
    validate::{ValidationConfig, ValidationError},
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Group {
    type Brep = FaceSet;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        objects: &Objects,
        debug_info: &mut DebugInfo,
    ) -> Result<Self::Brep, ValidationError> {
        let mut faces = FaceSet::new();

        let a = self.a.compute_brep(config, objects, debug_info)?;
        let b = self.b.compute_brep(config, objects, debug_info)?;

        faces.extend(a);
        faces.extend(b);

        Ok(faces)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
