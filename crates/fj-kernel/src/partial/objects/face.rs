use fj_interop::mesh::Color;

use crate::{
    objects::{Cycle, Face, Objects, Surface},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
    storage::Handle,
};

/// A partial [`Face`]
#[derive(Clone, Debug)]
pub struct PartialFace {
    /// The surface that the face is defined in
    pub surface: Handle<Surface>,

    /// The cycle that bounds the face on the outside
    pub exterior: Handle<Cycle>,

    /// The cycles that bound the face on the inside
    ///
    /// Each of these cycles defines a hole in the face.
    pub interiors: Vec<Handle<Cycle>>,

    /// The color of the face
    pub color: Option<Color>,
}

impl PartialObject for PartialFace {
    type Full = Face;

    fn new(_: &mut Service<Objects>) -> Self {
        // `PartialFace` is being phased out, and this method is no longer used.
        unreachable!()
    }

    fn from_full(face: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            surface: face.surface().clone(),
            exterior: face.exterior().clone(),
            interiors: face.interiors().cloned().collect(),
            color: Some(face.color()),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let surface = self.surface;
        let color = self.color.unwrap_or_default();

        Face::new(surface, self.exterior, self.interiors, color)
    }
}
