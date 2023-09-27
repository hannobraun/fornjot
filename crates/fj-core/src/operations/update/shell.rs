use crate::{
    objects::{Face, Shell},
    storage::Handle,
};

/// Update a [`Shell`]
pub trait UpdateShell {
    /// Add faces to the shell
    #[must_use]
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self;

    /// Update a face of the shell
    ///
    /// # Panics
    ///
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn update_face(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>) -> Handle<Face>,
    ) -> Self;

    /// Replace a face of the shell
    ///
    /// This is a more general version of [`UpdateShell::update_face`] which can
    /// replace a single face with multiple others.
    ///
    /// # Panics
    ///
    /// Uses [`Handles::replace`] internally, and panics for the same reasons.
    ///
    /// [`Handles::replace`]: crate::objects::Handles::replace
    #[must_use]
    fn replace_face<const N: usize>(
        &self,
        handle: &Handle<Face>,
        replace: impl FnOnce(&Handle<Face>) -> [Handle<Face>; N],
    ) -> Self;

    /// Remove a face from the shell
    #[must_use]
    fn remove_face(&self, handle: &Handle<Face>) -> Self;
}

impl UpdateShell for Shell {
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        let faces = self.faces().iter().cloned().chain(faces);
        Shell::new(faces)
    }

    fn update_face(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>) -> Handle<Face>,
    ) -> Self {
        let faces = self.faces().update(handle, update);
        Shell::new(faces)
    }

    fn replace_face<const N: usize>(
        &self,
        handle: &Handle<Face>,
        replace: impl FnOnce(&Handle<Face>) -> [Handle<Face>; N],
    ) -> Self {
        let faces = self.faces().replace(handle, replace);
        Shell::new(faces)
    }

    fn remove_face(&self, handle: &Handle<Face>) -> Self {
        let faces = self
            .faces()
            .iter()
            .filter(|face| face.id() == handle.id())
            .cloned();

        Shell::new(faces)
    }
}
