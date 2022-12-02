use crate::{
    objects::{Face, Objects, Shell},
    partial2::{Partial, PartialObject},
    services::Service,
};

/// A partial [`Shell`]
#[derive(Clone, Debug, Default)]
pub struct PartialShell {
    /// The faces that make up the shell
    pub faces: Vec<Partial<Face>>,
}

impl PartialObject for PartialShell {
    type Full = Shell;

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let faces = self.faces.into_iter().map(|face| face.build(objects));
        Shell::new(faces)
    }
}
