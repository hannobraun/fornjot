use fj_interop::mesh::Color;

use crate::{
    builder::CycleBuilder,
    objects::{Cycle, Face, Objects, Surface},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Face`]
#[derive(Clone, Debug, Default)]
pub struct PartialFace {
    /// The surface that the face is defined in
    pub surface: Partial<Surface>,

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

    fn from_full(face: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            surface: Partial::from_full(face.surface().clone(), cache),
            exterior: Partial::from_full(face.exterior().clone(), cache),
            interiors: face
                .interiors()
                .map(|cycle| Partial::from_full(cycle.clone(), cache))
                .collect(),
            color: Some(face.color()),
        }
    }

    fn build(mut self, objects: &mut Service<Objects>) -> Self::Full {
        let surface = self.surface.build(objects);

        self.exterior
            .write()
            .infer_vertex_positions_if_necessary(&surface.geometry());
        for interior in &mut self.interiors {
            interior
                .write()
                .infer_vertex_positions_if_necessary(&surface.geometry());
        }

        let exterior = self.exterior.build(objects);
        let interiors =
            self.interiors.into_iter().map(|cycle| cycle.build(objects));
        let color = self.color.unwrap_or_default();

        Face::new(surface, exterior, interiors, color)
    }
}
