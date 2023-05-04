use crate::{
    objects::{Face, Shell},
    storage::Handle,
};

/// Update a [`Shell`]
pub trait UpdateShell {
    /// Update a face of the shell
    fn update_face(
        &self,
        handle: &Handle<Face>,
        replacement: Handle<Face>,
    ) -> Shell;

    /// Remove a face from the shell
    fn remove_face(&self, handle: &Handle<Face>) -> Shell;
}

impl UpdateShell for Shell {
    fn update_face(
        &self,
        handle: &Handle<Face>,
        replacement: Handle<Face>,
    ) -> Shell {
        let faces = self.faces().into_iter().map(|face| {
            if face.id() == handle.id() {
                replacement.clone()
            } else {
                face.clone()
            }
        });

        Shell::new(faces)
    }

    fn remove_face(&self, handle: &Handle<Face>) -> Shell {
        let faces = self
            .faces()
            .into_iter()
            .filter(|face| face.id() == handle.id())
            .cloned();

        Shell::new(faces)
    }
}
