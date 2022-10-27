use fj_math::Point;

use crate::{
    objects::{Face, Faces, Objects, Sketch, Surface},
    storage::Handle,
};

/// API for building a [`Sketch`]
///
/// Also see [`Sketch::builder`].
pub struct SketchBuilder<'a> {
    /// The stores that the created objects are put in
    pub objects: &'a Objects,

    /// The surface that the [`Sketch`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The faces that make up the [`Sketch`]
    pub faces: Faces,
}

impl<'a> SketchBuilder<'a> {
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
    ) -> Self {
        let surface = self
            .surface
            .as_ref()
            .expect("Can't build `Sketch` without `Surface`");
        self.faces.extend([Face::builder(self.objects)
            .with_surface(surface.clone())
            .with_exterior_polygon_from_points(points)
            .build()]);
        self
    }

    /// Build the [`Sketch`]
    pub fn build(self) -> Handle<Sketch> {
        self.objects.sketches.insert(Sketch::new(self.faces))
    }
}
