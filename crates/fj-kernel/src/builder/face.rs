use std::ops::Deref;

use fj_math::Point;

use crate::{
    objects::{Cycle, Face, Surface},
    stores::Stores,
};

/// API for building a [`Face`]
///
/// Also see [`Face::builder`].
pub struct FaceBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Face`] is defined in
    pub surface: Surface,
}

impl<'a> FaceBuilder<'a> {
    /// Construct a polygon from a list of points
    pub fn build_polygon_from_points(
        &self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> FacePolygon {
        let cycle = Cycle::builder(self.stores, self.surface)
            .build_polygon_from_points(points);
        let face = Face::new(self.surface, cycle);

        FacePolygon {
            stores: self.stores,
            face,
        }
    }
}

/// A polygon
#[derive(Clone, Debug)]
pub struct FacePolygon<'a> {
    stores: &'a Stores,
    face: Face,
}

impl FacePolygon<'_> {
    /// Add a hole to the polygon
    pub fn with_hole(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let surface = *self.face.surface();
        self.face =
            self.face
                .with_interiors([Cycle::builder(self.stores, surface)
                    .build_polygon_from_points(points)]);

        self
    }

    /// Consume the `Polygon` and return the [`Face`] it wraps
    pub fn into_face(self) -> Face {
        self.face
    }
}

impl From<FacePolygon<'_>> for Face {
    fn from(polygon: FacePolygon) -> Self {
        polygon.into_face()
    }
}

impl Deref for FacePolygon<'_> {
    type Target = Face;

    fn deref(&self) -> &Self::Target {
        &self.face
    }
}
