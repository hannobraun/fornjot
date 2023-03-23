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
        f: impl FnMut(&Handle<Face>) -> Handle<Face>,
    ) -> Shell;
}

impl UpdateShell for Shell {
    fn update_face(
        &self,
        handle: &Handle<Face>,
        mut f: impl FnMut(&Handle<Face>) -> Handle<Face>,
    ) -> Shell {
        let faces = self.faces().into_iter().map(|face| {
            if face.id() == handle.id() {
                f(face)
            } else {
                face.clone()
            }
        });

        Shell::new(faces)
    }
}
