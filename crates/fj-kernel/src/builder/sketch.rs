use crate::{
    objects::{Face, Surface},
    partial::{Partial, PartialFace, PartialSketch},
};

/// Builder API for [`PartialSketch`]
pub trait SketchBuilder {
    /// Add a polygon to the sketch, created from the provided points
    fn add_face(
        &mut self,
        surface: impl Into<Partial<Surface>>,
    ) -> Partial<Face>;
}

impl SketchBuilder for PartialSketch {
    fn add_face(
        &mut self,
        surface: impl Into<Partial<Surface>>,
    ) -> Partial<Face> {
        let mut face = PartialFace::default();
        face.exterior.write().surface = surface.into();

        let face = Partial::from_partial(face);
        self.faces.extend([face.clone()]);

        face
    }
}
