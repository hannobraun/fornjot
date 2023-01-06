use crate::{
    objects::Face,
    partial::{Partial, PartialShell},
};

/// Builder API for [`PartialShell`]
pub trait ShellBuilder {
    /// Add a face to the shell
    ///
    /// The face will not be connected to any other faces that the shell might
    /// already have.
    fn add_face(&mut self) -> Partial<Face>;
}

impl ShellBuilder for PartialShell {
    fn add_face(&mut self) -> Partial<Face> {
        let face = Partial::default();
        self.faces.push(face.clone());
        face
    }
}
