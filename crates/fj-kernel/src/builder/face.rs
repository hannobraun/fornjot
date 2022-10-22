use fj_math::Point;

use crate::{
    objects::{Cycle, Face, Objects, Surface},
    partial::HasPartial,
    storage::Handle,
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
    pub exterior: Option<Handle<Cycle>>,

    /// The interior cycles that form holes in the [`Face`]
    pub interiors: Vec<Handle<Cycle>>,
}

impl<'a> FaceBuilder<'a> {
    /// Build the [`Face`] with an exterior polygon from the provided points
    pub fn with_exterior_polygon_from_points(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        self.exterior = Some(
            Cycle::partial()
                .with_surface(self.surface.clone())
                .with_poly_chain_from_points(points)
                .close_with_line_segment()
                .build(self.objects),
        );
        self
    }

    /// Build the [`Face`] with an interior polygon from the provided points
    pub fn with_interior_polygon_from_points(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        self.interiors.push(
            Cycle::partial()
                .with_surface(self.surface.clone())
                .with_poly_chain_from_points(points)
                .close_with_line_segment()
                .build(self.objects),
        );
        self
    }

    /// Construct a polygon from a list of points
    pub fn build(self) -> Face {
        let exterior = self
            .exterior
            .expect("Can't build `Face` without exterior cycle");
        Face::new(exterior).with_interiors(self.interiors)
    }
}
