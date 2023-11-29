use fj_math::Vector;

use crate::{
    objects::{Face, Shell},
    operations::insert::Insert,
    services::Services,
    storage::Handle,
};

use super::{SweepCache, SweepRegion};

/// # Sweep a [`Face`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepFace {
    /// # Sweep the [`Face`]
    fn sweep_face(
        &self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Shell;
}

impl SweepFace for Handle<Face> {
    fn sweep_face(
        &self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Shell {
        // Please note that this function uses the words "bottom" and "top" in a
        // specific sense:
        //
        // - "Bottom" refers to the origin of the sweep. The bottom face is the
        //   original face, or a face in the same place.
        // - "Top" refers to the location of the face that was created by
        //   translating the bottom face along the path.
        // - "Side" refers to new faces created in between bottom and top.
        //
        // These words are specifically *not* meant in the sense of z-axis
        // locations, and depending on the direction of `path`, the two meanings
        // might actually be opposite.

        let path = path.into();

        let mut faces = Vec::new();

        let bottom_face = self.clone();
        faces.push(bottom_face.clone());

        let side_faces = bottom_face
            .region()
            .sweep_region(bottom_face.surface(), path, cache, services)
            .into_iter()
            .map(|side_face| side_face.insert(services));
        faces.extend(side_faces);

        Shell::new(faces)
    }
}
