use fj_interop::mesh::Color;

use crate::{
    objects::{Cycle, Face, Objects, Surface},
    partial::{MaybePartial, MergeWith, Mergeable},
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`Face`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialFace {
    surface: Option<Handle<Surface>>,
    exterior: MaybePartial<Cycle>,
    interiors: Vec<MaybePartial<Cycle>>,
    color: Option<Color>,
}

impl PartialFace {
    /// Access th surface that the [`Face`] is defined in
    pub fn surface(&self) -> Option<Handle<Surface>> {
        self.surface.clone()
    }

    /// Access the [`Face`]'s exterior cycle
    pub fn exterior(&self) -> MaybePartial<Cycle> {
        self.exterior.clone()
    }

    /// Access the [`Face`]'s interior cycles
    pub fn interiors(&self) -> impl Iterator<Item = MaybePartial<Cycle>> + '_ {
        self.interiors.iter().cloned()
    }

    /// Access the color of the [`Face`]
    pub fn color(&self) -> Option<Color> {
        self.color
    }

    /// Build the [`Face`] with the provided surface
    pub fn with_surface(mut self, surface: Handle<Surface>) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Build the [`Face`] with the provided exterior
    pub fn with_exterior(
        mut self,
        exterior: impl Into<MaybePartial<Cycle>>,
    ) -> Self {
        self.exterior = exterior.into();
        self
    }

    /// Build the [`Face`] with the provided interior polygons
    pub fn with_interiors(
        mut self,
        interiors: impl IntoIterator<Item = impl Into<MaybePartial<Cycle>>>,
    ) -> Self {
        let interiors = interiors.into_iter().map(Into::into);
        self.interiors.extend(interiors);
        self
    }

    /// Build the [`Face`] with the provided color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Construct a polygon from a list of points
    pub fn build(self, objects: &mut Objects) -> Result<Face, ValidationError> {
        let exterior = self.exterior.into_full(objects)?;
        let interiors = self
            .interiors
            .into_iter()
            .map(|cycle| cycle.into_full(objects))
            .collect::<Result<Vec<_>, _>>()?;
        let color = self.color.unwrap_or_default();

        Ok(Face::new(exterior, interiors, color))
    }
}

impl MergeWith for PartialFace {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            surface: self.surface.merge_with(other.surface),
            exterior: self.exterior.merge_with(other.exterior),
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
            surface: Some(face.surface().clone()),
            exterior: face.exterior().clone().into(),
            interiors: face.interiors().cloned().map(Into::into).collect(),
            color: Some(face.color()),
        }
    }
}
