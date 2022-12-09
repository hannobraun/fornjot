use crate::{
    objects::{Face, Objects, Sketch},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Sketch`]
#[derive(Clone, Debug)]
pub struct PartialSketch {
    /// The faces that make up the sketch
    pub faces: Vec<Partial<Face>>,
}

impl PartialSketch {
    /// Construct an instance of `PartialSketch`
    pub fn new(faces: Vec<Partial<Face>>) -> Self {
        Self { faces }
    }
}

impl PartialObject for PartialSketch {
    type Full = Sketch;

    fn from_full(sketch: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self::new(
            sketch
                .faces()
                .into_iter()
                .map(|face| Partial::from_full(face.clone(), cache))
                .collect(),
        )
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let faces = self.faces.into_iter().map(|face| face.build(objects));
        Sketch::new(faces)
    }
}

impl Default for PartialSketch {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
