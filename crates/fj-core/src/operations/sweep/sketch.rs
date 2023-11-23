use fj_math::Vector;

use crate::{
    objects::{Face, Sketch, Solid, Surface},
    operations::insert::Insert,
    services::Services,
    storage::Handle,
};

use super::{face::SweepFace, SweepCache};

/// # Sweep a [`Sketch`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepSketch {
    /// # Sweep the [`Sketch`]
    fn sweep_sketch(
        &self,
        surface: Handle<Surface>,
        path: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Solid;
}

impl SweepSketch for Sketch {
    fn sweep_sketch(
        &self,
        surface: Handle<Surface>,
        path: impl Into<Vector<3>>,
        services: &mut Services,
    ) -> Solid {
        let path = path.into();
        let mut cache = SweepCache::default();

        let mut shells = Vec::new();
        for region in self.regions() {
            let face =
                Face::new(surface.clone(), region.clone()).insert(services);
            let shell =
                face.sweep_face(path, &mut cache, services).insert(services);
            shells.push(shell);
        }

        Solid::new(shells)
    }
}
