use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::{Face, Shell},
};

/// Update a [`Shell`]
pub trait UpdateShell {
    /// Add faces to the shell
    #[must_use]
    fn add_faces<T>(
        &self,
        faces: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Face>>;

    /// Update a face of the shell
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in multiple handles referencing the same object.
    #[must_use]
    fn update_face<T, R>(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Face>>,
        R: IntoIterator<Item = T>;

    /// Remove a face from the shell
    #[must_use]
    fn remove_face(&self, handle: &Handle<Face>) -> Self;
}

impl UpdateShell for Shell {
    fn add_faces<T>(
        &self,
        faces: impl IntoIterator<Item = T>,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Face>>,
    {
        let faces = faces.into_iter().map(|face| face.insert(core));
        let faces = self.faces().iter().cloned().chain(faces);
        Shell::new(faces)
    }

    fn update_face<T, R>(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>, &mut Core) -> R,
        core: &mut Core,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Face>>,
        R: IntoIterator<Item = T>,
    {
        let faces = self
            .faces()
            .replace(
                handle,
                update(handle, core).into_iter().map(|object| {
                    object.insert(core).derive_from(handle, core)
                }),
            )
            .expect("Face not found");
        Shell::new(faces)
    }

    fn remove_face(&self, handle: &Handle<Face>) -> Self {
        let faces = self
            .faces()
            .iter()
            .filter(|face| face.id() != handle.id())
            .cloned();

        Shell::new(faces)
    }
}
