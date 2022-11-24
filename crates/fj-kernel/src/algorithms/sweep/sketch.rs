use fj_math::Vector;

use crate::{
    objects::{Objects, Sketch, Solid},
    storage::Handle,
    validate::ValidationError,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Sketch> {
    type Swept = Handle<Solid>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Objects,
    ) -> Result<Self::Swept, ValidationError> {
        let path = path.into();

        let mut shells = Vec::new();
        for face in self.faces().clone() {
            let shell = face.sweep_with_cache(path, cache, objects)?;
            shells.push(shell);
        }

        Ok(Solid::builder().with_shells(shells).build(objects))
    }
}
