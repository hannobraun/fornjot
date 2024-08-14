use fj_math::Vector;

use crate::{
    operations::insert::Insert,
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
    ///
    /// Requires `path` to point towards the back of `surface`. If one of them
    /// is fixed, make sure to adapt the other one accordingly.
    ///
    /// Not following this requirement will produce an invalid shape that
    /// _should_ fail validation.
    ///
    /// ## Implementation Note
    ///
    /// The above requirement is a bit draconian. It would be much nicer, if
    /// this operation just worked, regardless of the relation of `path` and
    /// `surface`, and in fact, a previous version of it did.
    ///
    /// However, this previous version also made some undocumented assumption
    /// that didn't hold in the general case. It was also getting in the way of
    /// introducing the new geometry system.
    ///
    /// The decision was made that, for now, simplifying this operation and
    /// putting more requirements on the caller, was the right call. Once the
    /// new geometry system is in place, we'll hopefully be in a position to
    /// improve the sweep operation substantially.
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
