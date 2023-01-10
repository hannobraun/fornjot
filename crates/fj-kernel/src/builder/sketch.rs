use fj_math::Point;

use crate::{
    objects::Surface,
    partial::{Partial, PartialFace, PartialSketch},
};

use super::CycleBuilder;

/// Builder API for [`PartialSketch`]
pub trait SketchBuilder {
    /// Add a polygon to the sketch, created from the provided points
    fn add_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );
}

impl SketchBuilder for PartialSketch {
    fn add_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) {
        let mut face = PartialFace::default();
        face.exterior.write().surface = surface.into();
        face.exterior.write().update_as_polygon_from_points(points);

        let face = Partial::from_partial(face);
        self.faces.extend([face]);
    }
}
