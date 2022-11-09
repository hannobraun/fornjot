use fj_interop::mesh::Color;
use fj_math::Point;

use crate::{
    builder::CycleBuilder,
    objects::{Cycle, Face, Objects, Surface},
    partial::{util::merge_options, HasPartial, MaybePartial},
    storage::Handle,
    validate::ValidationError,
};

/// API for building a [`Face`]
///
/// Also see [`Face::builder`].
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

    /// Build the [`Face`] with an exterior polygon from the provided points
    pub fn with_exterior_polygon_from_points(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let surface = self
            .surface
            .as_ref()
            .expect("Need surface to create polygon");

        self.exterior = Cycle::partial()
            .with_poly_chain_from_points(surface.clone(), points)
            .close_with_line_segment()
            .into();
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

    /// Build the [`Face`] with an interior polygon from the provided points
    pub fn with_interior_polygon_from_points(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let surface = self
            .surface
            .as_ref()
            .expect("Need surface to build polygon.");

        self.interiors.push(
            Cycle::partial()
                .with_poly_chain_from_points(surface.clone(), points)
                .close_with_line_segment()
                .into(),
        );
        self
    }

    /// Build the [`Face`] with the provided color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Merge this partial object with another
    pub fn merge_with(self, other: Self) -> Self {
        let mut interiors = self.interiors;
        interiors.extend(other.interiors);

        Self {
            surface: merge_options(self.surface, other.surface),
            exterior: self.exterior.merge_with(other.exterior),
            interiors,
            color: merge_options(self.color, other.color),
        }
    }

    /// Construct a polygon from a list of points
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<Face>, ValidationError> {
        let exterior = self.exterior.into_full(objects)?;
        let interiors = self
            .interiors
            .into_iter()
            .map(|cycle| cycle.into_full(objects))
            .collect::<Result<Vec<_>, _>>()?;
        let color = self.color.unwrap_or_default();

        Ok(objects
            .faces
            .insert(Face::new(exterior, interiors, color))?)
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
