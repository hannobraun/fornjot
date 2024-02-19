use fj_math::Vector;

use crate::{
    objects::{Face, Region, Shell},
    operations::{
        derive::DeriveFrom,
        insert::Insert,
        presentation::GetColor,
        reverse::Reverse,
        sweep::{SweepCache, SweepRegion},
        update::UpdateShell,
    },
    storage::Handle,
    Core,
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
    ) -> Self;
}

impl SweepFaceOfShell for Shell {
    fn sweep_face_of_shell(
        &self,
        face: Handle<Face>,
        path: impl Into<Vector<3>>,
        core: &mut Core,
    ) -> Self {
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
        let region = Region::new(exterior, [], face.region().color());
        let faces = region
            .sweep_region(
                face.surface(),
                face.region().get_color(core),
                path,
                &mut cache,
                core,
            )
            .all_faces()
            .collect::<Vec<_>>();

        self.remove_face(&face).add_faces(faces, core)
    }
}
