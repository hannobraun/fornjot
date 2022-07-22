use fj_math::Point;

use crate::objects::{Cycle, Face, Surface};

/// API for building a [`Face`]
pub struct FaceBuilder {
    surface: Surface,
}

impl FaceBuilder {
    /// Construct an instance of `FaceBuilder`
    ///
    /// Also see [`Face::build`].
    pub fn new(surface: Surface) -> Self {
        Self { surface }
    }

    /// Construct a polygon from a list of points
    pub fn polygon_from_points(
        &self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Face {
        Face::new(self.surface)
            .with_exteriors([Cycle::polygon_from_points(&self.surface, points)])
    }
}
