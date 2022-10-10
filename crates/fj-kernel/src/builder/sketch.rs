use fj_math::Point;

use crate::{
    objects::{Face, Sketch, Stores, Surface},
    storage::Handle,
};

/// API for building a [`Sketch`]
///
/// Also see [`Sketch::builder`].
pub struct SketchBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Sketch`] is defined in
    pub surface: Handle<Surface>,
}

impl<'a> SketchBuilder<'a> {
    /// Construct a polygon from a list of points
    pub fn build_polygon_from_points(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Sketch {
        let face = Face::builder(self.stores, self.surface)
            .with_exterior_polygon_from_points(points)
            .build();
        Sketch::new().with_faces([face])
    }
}
