use fj_math::Point;

use crate::objects::{Face, Sketch, Surface};

/// API for building a [`Sketch`]
pub struct SketchBuilder {
    surface: Surface,
}

impl SketchBuilder {
    /// Construct an instance of `SketchBuilder`
    ///
    /// Also see [`Sketch::build`].
    pub fn new(surface: Surface) -> Self {
        Self { surface }
    }

    /// Construct a polygon from a list of points
    pub fn polygon_from_points(
        &self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Sketch {
        let face = Face::build(self.surface).polygon_from_points(points);
        Sketch::new().with_faces([face])
    }
}
