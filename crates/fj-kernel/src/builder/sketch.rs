use fj_math::Point;

use crate::{
    objects::Surface,
    partial::{Partial, PartialFace, PartialSketch},
    storage::Handle,
};

use super::FaceBuilder;

/// Builder API for [`PartialSketch`]
pub trait SketchBuilder {
    /// Construct a polygon from a list of points
    fn with_polygon_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;
}

impl SketchBuilder for PartialSketch {
    fn with_polygon_from_points(
        mut self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let mut face = PartialFace::default();
        face.with_exterior_polygon_from_points(surface, points);

        self.faces.extend([Partial::from_partial(face)]);
        self
    }
}
