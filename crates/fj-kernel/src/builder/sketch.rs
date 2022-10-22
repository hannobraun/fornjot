use fj_math::Point;

use crate::{
    objects::{Face, Objects, Sketch, Surface},
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
}

impl<'a> SketchBuilder<'a> {
    /// Build the [`Sketch`] with the provided [`Surface`]
    pub fn with_surface(mut self, surface: Handle<Surface>) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Construct a polygon from a list of points
    pub fn build_polygon_from_points(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Sketch {
        let surface = self
            .surface
            .expect("Can't build `Sketch` without `Surface`");
        let face = Face::builder(self.objects)
            .with_surface(surface)
            .with_exterior_polygon_from_points(points)
            .build();
        Sketch::new().with_faces([face])
    }
}
