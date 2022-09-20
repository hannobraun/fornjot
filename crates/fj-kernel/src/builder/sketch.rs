use fj_math::Point;

use crate::{
    objects::{Face, Sketch, Surface},
    stores::Stores,
};

/// API for building a [`Sketch`]
///
/// Also see [`Sketch::builder`].
pub struct SketchBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Sketch`] is defined in
    pub surface: Surface,
}

impl<'a> SketchBuilder<'a> {
    /// Construct a polygon from a list of points
    pub fn polygon_from_points(
        &self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Sketch {
        let face = Face::build(self.stores, self.surface)
            .polygon_from_points(points)
            .into_face();
        Sketch::new().with_faces([face])
    }
}
