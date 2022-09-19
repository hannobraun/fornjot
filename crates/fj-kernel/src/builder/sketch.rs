use fj_math::Point;

use crate::{
    objects::{Face, Sketch, Surface},
    stores::Stores,
};

/// API for building a [`Sketch`]
pub struct SketchBuilder<'a> {
    stores: &'a Stores,
    surface: Surface,
}

impl<'a> SketchBuilder<'a> {
    /// Construct an instance of `SketchBuilder`
    ///
    /// Also see [`Sketch::build`].
    pub fn new(stores: &'a Stores, surface: Surface) -> Self {
        Self { stores, surface }
    }

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
