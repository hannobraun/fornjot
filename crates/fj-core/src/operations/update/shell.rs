use crate::{
    objects::{Face, Shell},
    storage::Handle,
};

/// Update a [`Shell`]
pub trait UpdateShell {
    /// Add faces to the shell
    #[must_use]
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self;

    /// Replace a face of the shell
    #[must_use]
    fn update_face(
        &self,
        face: &Handle<Face>,
        replacement: Handle<Face>,
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
        face: &Handle<Face>,
        replacement: Handle<Face>,
    ) -> Self {
        let faces = self.faces().iter().map(|f| {
            if f.id() == face.id() {
                replacement.clone()
            } else {
                f.clone()
            }
        });

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
