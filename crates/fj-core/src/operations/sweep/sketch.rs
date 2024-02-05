use fj_math::{Scalar, Vector};

use crate::{
    geometry::GlobalPath,
    objects::{Face, Sketch, Solid, Surface},
    operations::{insert::Insert, reverse::Reverse},
    storage::Handle,
    Instance,
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
        core: &mut Instance,
    ) -> Solid;
}

impl SweepSketch for Sketch {
    fn sweep_sketch(
        &self,
        surface: Handle<Surface>,
        path: impl Into<Vector<3>>,
        core: &mut Instance,
    ) -> Solid {
        let path = path.into();
        let mut cache = SweepCache::default();

        let mut shells = Vec::new();
        for region in self.regions() {
            let region = {
                // The following code assumes that the sketch is winded counter-
                // clockwise. Let's check that real quick.
                assert!(region.exterior().winding().is_ccw());

                let is_negative_sweep = {
                    let u = match surface.geometry().u {
                        GlobalPath::Circle(_) => todo!(
                            "Sweeping sketch from a rounded surfaces is not \
                            supported"
                        ),
                        GlobalPath::Line(line) => line.direction(),
                    };
                    let v = surface.geometry().v;

                    let normal = u.cross(&v);

                    normal.dot(&path) < Scalar::ZERO
                };

                if is_negative_sweep {
                    region.clone()
                } else {
                    region
                        .reverse(&mut core.services)
                        .insert(&mut core.services)
                }
            };

            let face = Face::new(surface.clone(), region.clone())
                .insert(&mut core.services);
            let shell = face
                .sweep_face(path, &mut cache, &mut core.services)
                .insert(&mut core.services);
            shells.push(shell);
        }

        Solid::new(shells)
    }
}
