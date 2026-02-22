use crate::{
    Core,
    math::Vector,
    operations::{
        derive::DeriveFrom,
        insert::Insert,
        presentation::GetColor,
        reverse::Reverse,
        sweep::{SweepCache, SweepRegion},
        update::UpdateShell,
    },
    storage::Handle,
    topology::{Face, Region, Shell},
};

/// # Sweep a [`Face`] that is part of a [`Shell`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepFaceOfShell {
    /// # Sweep the [`Face`] of the [`Shell`]
    ///
    /// Extends the shell, adding the new faces to it.
    ///
    /// # Limitation
    ///
    ///  When generating new faces, these must NOT coincide with any existing faces in the shell.
    ///
    /// # Panics
    ///
    /// Panics, if the face has interior cycles. This is not a fundamental
    /// limitation, but none the less not yet supported.
    fn sweep_face_of_shell(
        &self,
        face: Handle<Face>,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> ShellExtendedBySweep;
}

impl SweepFaceOfShell for Shell {
    fn sweep_face_of_shell(
        &self,
        face: Handle<Face>,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> ShellExtendedBySweep {
        let path = path.into();

        if !face.region().interiors().is_empty() {
            todo!(
                "Sweeping shell faces with interior cycles is not yet \
                supported."
            )
        }

        let mut cache = SweepCache::default();

        let exterior = face
            .region()
            .exterior()
            .reverse(core)
            .insert(core)
            .derive_from(face.region().exterior(), core);
        let region = Region::new(exterior, []);
        let swept_region = region.sweep_region(
            face.surface().clone(),
            face.region().get_color(core),
            path,
            &mut cache,
            core,
        );

        let shell = self
            .remove_face(&face)
            .add_faces(swept_region.clone().all_faces(), core);

        ShellExtendedBySweep {
            shell,
            side_faces: swept_region.side_faces,
            top_face: swept_region.top_face,
        }
    }
}

/// The result of sweeping a [`Face`] of a [`Shell`]
///
/// See [`SweepFaceOfShell`].
pub struct ShellExtendedBySweep {
    /// The resulting shell after the sweep
    pub shell: Shell,

    /// The side faces created by the sweep
    pub side_faces: Vec<Face>,

    /// The top face created by the sweep
    pub top_face: Face,
}
