use crate::{
    objects::{Face, Objects, Shell},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
    storage::Handle,
};

/// A partial [`Shell`]
#[derive(Clone, Debug)]
pub struct PartialShell {
    /// The faces that make up the shell
    pub faces: Vec<Handle<Face>>,
}

impl PartialObject for PartialShell {
    type Full = Shell;

    fn new(_: &mut Service<Objects>) -> Self {
        Self { faces: Vec::new() }
    }

    fn from_full(shell: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            faces: shell.faces().into_iter().cloned().collect(),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let faces = self.faces;
        Shell::new(faces)
    }
}
