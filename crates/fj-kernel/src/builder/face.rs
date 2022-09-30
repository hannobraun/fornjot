use fj_math::Point;

use crate::{
    objects::{Cycle, Face, Surface},
    stores::{Handle, Stores},
};

/// API for building a [`Face`]
///
/// Also see [`Face::builder`].
pub struct FaceBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Face`] is defined in
    pub surface: Handle<Surface>,

    /// The exterior cycle that bounds the [`Face`] on the outside
    ///
    /// Must be provided by the caller, directly or using one of the `with_`
    /// methods, before [`FaceBuilder::build`] is called.
    pub exterior: Option<Cycle>,

    /// The interior cycles that form holes in the [`Face`]
    pub interiors: Vec<Cycle>,
}

impl<'a> FaceBuilder<'a> {
    /// Build the [`Face`] with an exterior polygon from the provided points
    pub fn with_exterior_polygon_from_points(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        self.exterior = Some(
            Cycle::builder(self.stores, self.surface.clone())
                .with_poly_chain_from_points(points)
                .close_with_line_segment()
                .build(),
        );
        self
    }

    /// Build the [`Face`] with an interior polygon from the provided points
    pub fn with_interior_polygon_from_points(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        self.interiors.push(
            Cycle::builder(self.stores, self.surface.clone())
                .with_poly_chain_from_points(points)
                .close_with_line_segment()
                .build(),
        );
        self
    }

    /// Construct a polygon from a list of points
    pub fn build(self) -> Face {
        let exterior = self
            .exterior
            .expect("Can't build `Face` without exterior cycle");
        Face::from_exterior(exterior).with_interiors(self.interiors)
    }
}
