use fj_interop::mesh::Color;

use crate::{
    objects::{Cycle, Face, Objects, Surface},
    partial::{MergeWith, Mergeable},
    partial2::Partial,
    services::Service,
};

/// A partial [`Face`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialFace {
    /// The [`Face`]'s exterior cycle
    pub exterior: Partial<Cycle>,

    /// The [`Face`]'s interior cycles
    pub interiors: Vec<Partial<Cycle>>,

    /// The color of the [`Face`]
    pub color: Option<Color>,
}

impl PartialFace {
    /// Access th surface that the [`Face`] is defined in
    pub fn surface(&self) -> Option<Partial<Surface>> {
        self.exterior.read().surface()
    }

    /// Construct a polygon from a list of points
    pub fn build(self, objects: &mut Service<Objects>) -> Face {
        let exterior = self.exterior.build(objects);
        let interiors = self
            .interiors
            .into_iter()
            .map(|cycle| cycle.build(objects))
            .collect::<Vec<_>>();
        let color = self.color.unwrap_or_default();

        Face::new(exterior, interiors, color)
    }
}

impl MergeWith for PartialFace {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            exterior: self.exterior,
            interiors: Mergeable(self.interiors)
                .merge_with(Mergeable(other.interiors))
                .0,
            color: self.color.merge_with(other.color),
        }
    }
}

impl From<&Face> for PartialFace {
    fn from(face: &Face) -> Self {
        Self {
            exterior: Partial::from_full_entry_point(face.exterior().clone()),
            interiors: face
                .interiors()
                .cloned()
                .map(Partial::from_full_entry_point)
                .collect(),
            color: Some(face.color()),
        }
    }
}
