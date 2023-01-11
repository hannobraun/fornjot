use crate::{
    objects::Face,
    partial::{Partial, PartialFace, PartialSketch},
};

/// Builder API for [`PartialSketch`]
pub trait SketchBuilder {
    /// Add a face to the sketch
    fn add_face(&mut self) -> Partial<Face>;
}

impl SketchBuilder for PartialSketch {
    fn add_face(&mut self) -> Partial<Face> {
        let face = PartialFace::default();

        let face = Partial::from_partial(face);
        self.faces.extend([face.clone()]);

        face
    }
}
