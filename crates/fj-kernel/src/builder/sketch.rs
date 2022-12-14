use fj_math::Point;

use crate::{
    objects::{Face, Objects, Surface},
    partial::{Partial, PartialFace, PartialSketch},
    services::Service,
    storage::Handle,
};

use super::FaceBuilder;

/// Builder API for [`PartialSketch`]
pub trait SketchBuilder {
    /// Build the [`Sketch`] with the provided faces
    fn with_faces(self, faces: impl IntoIterator<Item = Handle<Face>>) -> Self;

    /// Construct a polygon from a list of points
    fn with_polygon_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
        objects: &mut Service<Objects>,
    ) -> Self;
}

impl SketchBuilder for PartialSketch {
    fn with_faces(
        mut self,
        faces: impl IntoIterator<Item = Handle<Face>>,
    ) -> Self {
        let faces = faces.into_iter().map(Partial::from);
        self.faces.extend(faces);
        self
    }

    fn with_polygon_from_points(
        mut self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
        _: &mut Service<Objects>,
    ) -> Self {
        let mut face = PartialFace::default();
        face.with_exterior_polygon_from_points(surface, points);

        self.faces.extend([Partial::from_partial(face)]);
        self
    }
}
