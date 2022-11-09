use fj_interop::mesh::Color;
use fj_math::Point;

use crate::{
    builder::CycleBuilder,
    objects::{Cycle, Face, Objects, Surface},
    partial::{HasPartial, MaybePartial},
    storage::Handle,
    validate::ValidationError,
};

/// API for building a [`Face`]
///
/// Also see [`Face::builder`].
pub struct FaceBuilder<'a> {
    /// The stores that the created objects are put in
    pub objects: &'a Objects,

    /// The surface that the [`Face`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The exterior cycle that bounds the [`Face`] on the outside
    ///
    /// Must be provided by the caller, directly or using one of the `with_`
    /// methods, before [`FaceBuilder::build`] is called.
    pub exterior: MaybePartial<Cycle>,

    /// The interior cycles that form holes in the [`Face`]
    pub interiors: Vec<MaybePartial<Cycle>>,

    /// The color of the [`Face`]
    pub color: Option<Color>,
}

impl<'a> FaceBuilder<'a> {
    /// Build the [`Face`] with the provided surface
    pub fn with_surface(mut self, surface: Handle<Surface>) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Build the [`Face`] with the provided exterior
    pub fn with_exterior(mut self, exterior: Handle<Cycle>) -> Self {
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

    /// Construct a polygon from a list of points
    pub fn build(self) -> Result<Handle<Face>, ValidationError> {
        let exterior = self.exterior.into_full(self.objects)?;
        let interiors = self
            .interiors
            .into_iter()
            .map(|cycle| cycle.into_full(self.objects))
            .collect::<Result<Vec<_>, _>>()?;
        let color = self.color.unwrap_or_default();

        Ok(self
            .objects
            .faces
            .insert(Face::new(exterior, interiors, color))?)
    }
}
