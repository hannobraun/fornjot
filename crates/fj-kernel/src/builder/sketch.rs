use fj_math::Point;

use crate::{
    insert::Insert,
    objects::{Face, FaceSet, Objects, Sketch, Surface},
    partial::{Partial, PartialFace, PartialObject},
    services::Service,
    storage::Handle,
};

use super::FaceBuilder;

/// API for building a [`Sketch`]
///
/// Also see [`Sketch::builder`].
pub struct SketchBuilder {
    /// The faces that make up the [`Sketch`]
    pub faces: FaceSet,
}

impl SketchBuilder {
    /// Build the [`Sketch`] with the provided faces
    pub fn with_faces(
        mut self,
        faces: impl IntoIterator<Item = Handle<Face>>,
    ) -> Self {
        self.faces.extend(faces);
        self
    }

    /// Construct a polygon from a list of points
    pub fn with_polygon_from_points(
        mut self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
        objects: &mut Service<Objects>,
    ) -> Self {
        let mut face = PartialFace::default();
        face.with_exterior_polygon_from_points(
            Partial::from_full_entry_point(surface),
            points,
        );
        let face = face.build(objects).insert(objects);

        self.faces.extend([face]);
        self
    }

    /// Build the [`Sketch`]
    pub fn build(self, objects: &mut Service<Objects>) -> Handle<Sketch> {
        Sketch::new(self.faces).insert(objects)
    }
}
