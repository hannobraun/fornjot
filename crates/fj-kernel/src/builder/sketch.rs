use fj_math::Point;

use crate::{
    insert::Insert,
    objects::{Face, FaceSet, Objects, Sketch, Surface},
    partial::HasPartial,
    services::Service,
    storage::Handle,
};

use super::FaceBuilder;

/// API for building a [`Sketch`]
///
/// Also see [`Sketch::builder`].
pub struct SketchBuilder {
    /// The surface that the [`Sketch`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The faces that make up the [`Sketch`]
    pub faces: FaceSet,
}

impl SketchBuilder {
    /// Build the [`Sketch`] with the provided [`Surface`]
    pub fn with_surface(mut self, surface: Handle<Surface>) -> Self {
        self.surface = Some(surface);
        self
    }

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
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
        objects: &mut Service<Objects>,
    ) -> Self {
        let surface = self
            .surface
            .as_ref()
            .expect("Can't build `Sketch` without `Surface`");
        self.faces.extend([Face::partial()
            .with_surface(surface.clone())
            .with_exterior_polygon_from_points(points)
            .build(objects)
            .unwrap()
            .insert(objects)
            .unwrap()]);
        self
    }

    /// Build the [`Sketch`]
    pub fn build(self, objects: &mut Service<Objects>) -> Handle<Sketch> {
        Sketch::new(self.faces).insert(objects).unwrap()
    }
}
