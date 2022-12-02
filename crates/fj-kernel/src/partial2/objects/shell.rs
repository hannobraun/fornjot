use crate::{
    objects::{Face, Objects, Shell},
    partial2::{Partial, PartialObject},
    services::Service,
};

/// A partial [`Shell`]
#[derive(Clone, Debug)]
pub struct PartialShell {
    /// The faces that make up the shell
    pub faces: Vec<Partial<Face>>,
}

impl PartialShell {
    /// Construct an instance of `PartialShell`
    pub fn new(faces: Vec<Partial<Face>>) -> Self {
        Self { faces }
    }
}

impl PartialObject for PartialShell {
    type Full = Shell;

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let faces = self.faces.into_iter().map(|face| face.build(objects));
        Shell::new(faces)
    }
}

impl Default for PartialShell {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
