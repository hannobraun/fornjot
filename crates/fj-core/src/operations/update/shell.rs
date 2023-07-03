use crate::{
    objects::{Face, Shell},
    storage::Handle,
};

/// Update a [`Shell`]
pub trait UpdateShell {
    /// Add faces to the shell
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self;

    /// Update a face of the shell
    fn replace_face(
        &self,
        original: &Handle<Face>,
        replacement: Handle<Face>,
    ) -> Self;

    /// Remove a face from the shell
    fn remove_face(&self, handle: &Handle<Face>) -> Self;
}

impl UpdateShell for Shell {
    fn add_faces(&self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        let faces = self.faces().into_iter().cloned().chain(faces);
        Shell::new(faces)
    }

    fn replace_face(
        &self,
        original: &Handle<Face>,
        replacement: Handle<Face>,
    ) -> Self {
        let faces = self.faces().into_iter().map(|face| {
            if face.id() == original.id() {
                replacement.clone()
            } else {
                face.clone()
            }
        });

        Shell::new(faces)
    }

    fn remove_face(&self, handle: &Handle<Face>) -> Self {
        let faces = self
            .faces()
            .into_iter()
            .filter(|face| face.id() == handle.id())
            .cloned();

        Shell::new(faces)
    }
}
