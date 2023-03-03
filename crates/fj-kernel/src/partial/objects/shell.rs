use crate::{
    objects::{Face, Objects, Shell},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Shell`]
#[derive(Clone, Debug)]
pub struct PartialShell {
    /// The faces that make up the shell
    pub faces: Vec<Partial<Face>>,
}

impl PartialObject for PartialShell {
    type Full = Shell;

    fn new() -> Self {
        Self { faces: Vec::new() }
    }

    fn from_full(shell: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            faces: shell
                .faces()
                .into_iter()
                .map(|face| Partial::from_full(face.clone(), cache))
                .collect(),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let faces = self.faces.into_iter().map(|face| face.build(objects));
        Shell::new(faces)
    }
}
