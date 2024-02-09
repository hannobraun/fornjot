use crate::{
    objects::{Face, Shell},
    operations::insert::Insert,
    storage::Handle,
    Instance,
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
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_face<T, const N: usize>(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>, &mut Instance) -> [T; N],
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Face>>;

    /// Remove a face from the shell
    #[must_use]
    fn remove_face(&self, handle: &Handle<Face>) -> Self;
}

impl UpdateShell for Shell {
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        let faces = self.faces().iter().cloned().chain(faces);
        Shell::new(faces)
    }

    fn update_face<T, const N: usize>(
        &self,
        handle: &Handle<Face>,
        update: impl FnOnce(&Handle<Face>, &mut Instance) -> [T; N],
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Face>>,
    {
        let faces = self
            .faces()
            .replace(
                handle,
                update(handle, core)
                    .map(|object| object.insert(&mut core.services)),
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
