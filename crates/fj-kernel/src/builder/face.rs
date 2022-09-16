use std::ops::Deref;

use fj_math::Point;

use crate::{
    objects::{Cycle, Face, Surface},
    stores::Stores,
};

/// API for building a [`Face`]
pub struct FaceBuilder<'a> {
    stores: &'a Stores,
    surface: Surface,
}

impl<'a> FaceBuilder<'a> {
    /// Construct an instance of `FaceBuilder`
    ///
    /// Also see [`Face::build`].
    pub fn new(stores: &'a Stores, surface: Surface) -> Self {
        Self { stores, surface }
    }

    /// Construct a polygon from a list of points
    pub fn polygon_from_points(
        &self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> FacePolygon {
        let cycle =
            Cycle::build(self.stores, self.surface).polygon_from_points(points);
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
                .with_interiors([Cycle::build(self.stores, surface)
                    .polygon_from_points(points)]);

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
