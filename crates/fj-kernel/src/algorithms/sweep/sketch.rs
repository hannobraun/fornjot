use fj_math::Vector;

use crate::objects::{Objects, Sketch, Solid};

use super::{Sweep, SweepCache};

impl Sweep for Sketch {
    type Swept = Solid;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &Objects,
    ) -> Self::Swept {
        let path = path.into();

        let mut shells = Vec::new();
        for face in self.faces().clone() {
            let shell = face.sweep_with_cache(path, cache, objects);
            shells.push(shell);
        }

        Solid::new().with_shells(shells)
    }
}
