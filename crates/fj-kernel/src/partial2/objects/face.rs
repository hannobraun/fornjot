use fj_interop::mesh::Color;

use crate::{
    objects::{Cycle, Face, Objects},
    partial2::{Partial, PartialObject},
    services::Service,
};

/// A partial [`Face`]
#[derive(Clone, Debug, Default)]
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

impl PartialObject for PartialFace {
    type Full = Face;

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let exterior = self.exterior.build(objects);
        let interiors =
            self.interiors.into_iter().map(|cycle| cycle.build(objects));
        let color = self.color.unwrap_or_default();

        Face::new(exterior, interiors, color)
    }
}
