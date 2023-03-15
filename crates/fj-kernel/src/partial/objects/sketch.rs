use crate::{
    objects::{Face, Objects, Sketch},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
    storage::Handle,
};

/// A partial [`Sketch`]
#[derive(Clone, Debug)]
pub struct PartialSketch {
    /// The faces that make up the sketch
    pub faces: Vec<Handle<Face>>,
}

impl PartialObject for PartialSketch {
    type Full = Sketch;

    fn new(_: &mut Service<Objects>) -> Self {
        Self { faces: Vec::new() }
    }

    fn from_full(sketch: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            faces: sketch.faces().into_iter().cloned().collect(),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let faces = self.faces;
        Sketch::new(faces)
    }
}
