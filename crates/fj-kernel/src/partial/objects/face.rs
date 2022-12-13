use fj_interop::mesh::Color;

use crate::{
    objects::{Cycle, Face, Objects},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Face`]
#[derive(Clone, Debug)]
pub struct PartialFace {
    /// The cycle that bounds the face on the outside
    pub exterior: Partial<Cycle>,

    /// The cycles that bound the face on the inside
    ///
    /// Each of these cycles defines a hole in the face.
    pub interiors: Vec<Partial<Cycle>>,

    /// The color of the face
    pub color: Option<Color>,
}

impl PartialFace {
    /// Construct an instance of `PartialFace`
    pub fn new(
        exterior: Option<Partial<Cycle>>,
        interiors: Vec<Partial<Cycle>>,
        color: Option<Color>,
    ) -> Self {
        let exterior = exterior.unwrap_or_default();

        Self {
            exterior,
            interiors,
            color,
        }
    }
}

impl PartialObject for PartialFace {
    type Full = Face;

    fn from_full(face: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            exterior: Partial::from_full(face.exterior().clone(), cache),
            interiors: face
                .interiors()
                .map(|cycle| Partial::from_full(cycle.clone(), cache))
                .collect(),
            color: Some(face.color()),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let exterior = self.exterior.build(objects);
        let interiors =
            self.interiors.into_iter().map(|cycle| cycle.build(objects));
        let color = self.color.unwrap_or_default();

        Face::new(exterior, interiors, color)
    }
}

impl Default for PartialFace {
    fn default() -> Self {
        Self::new(None, Vec::new(), None)
    }
}
