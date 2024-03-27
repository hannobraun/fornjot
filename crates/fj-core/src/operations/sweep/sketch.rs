use fj_math::{Scalar, Vector};

use crate::{
    geometry::GlobalPath,
    operations::{derive::DeriveFrom, insert::Insert, reverse::Reverse},
    storage::Handle,
    topology::{Face, Sketch, Solid, Surface},
    Core,
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
        core: &mut Core,
    ) -> Solid;
}

impl SweepSketch for Sketch {
    fn sweep_sketch(
        &self,
        surface: Handle<Surface>,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Solid {
        let path = path.into();
        let mut cache = SweepCache::default();

        let mut shells = Vec::new();
        for region in self.regions() {
            let region = {
                // The following code assumes that the sketch is wound counter-
                // clockwise. Let's check that real quick.
                assert!(region
                    .exterior()
                    .winding(&core.layers.geometry, self.surface())
                    .is_ccw());

                let is_negative_sweep = {
                    let u = match core.layers.geometry.of_surface(&surface).u {
                        GlobalPath::Circle(_) => todo!(
                            "Sweeping sketch from a rounded surfaces is not \
                            supported"
                        ),
                        GlobalPath::Line(line) => line.direction(),
                    };
                    let v = core.layers.geometry.of_surface(&surface).v;

                    let normal = u.cross(&v);

                    normal.dot(&path) < Scalar::ZERO
                };

                if is_negative_sweep {
                    region.clone()
                } else {
                    region.reverse(core).insert(core).derive_from(region, core)
                }
            };

            for cycle in region.all_cycles() {
                for half_edge in cycle.half_edges() {
                    let curve_geom = core
                        .layers
                        .geometry
                        .of_curve(half_edge.curve())
                        .unwrap()
                        .local_on(self.surface())
                        .unwrap();

                    core.layers.geometry.define_curve(
                        half_edge.curve().clone(),
                        surface.clone(),
                        curve_geom.clone(),
                    );
                }
            }

            let face = Face::new(surface.clone(), region.clone()).insert(core);
            let shell = face.sweep_face(path, &mut cache, core).insert(core);
            shells.push(shell);
        }

        Solid::new(shells)
    }
}
