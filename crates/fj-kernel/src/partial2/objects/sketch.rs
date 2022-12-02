use crate::{
    objects::{Face, Objects, Sketch},
    partial2::{Partial, PartialObject},
    services::Service,
};

/// A partial [`Sketch`]
#[derive(Clone, Debug, Default)]
pub struct PartialSketch {
    /// The faces that make up the sketch
    pub faces: Vec<Partial<Face>>,
}

impl PartialObject for PartialSketch {
    type Full = Sketch;

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let faces = self.faces.into_iter().map(|face| face.build(objects));
        Sketch::new(faces)
    }
}
