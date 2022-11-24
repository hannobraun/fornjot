use crate::{
    builder::SketchBuilder,
    objects::{Face, FaceSet, Objects},
    storage::Handle,
};

/// A 2-dimensional shape
///
/// # Implementation Note
///
/// The faces that make up the sketch must be in the same surface. This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    faces: FaceSet,
}

impl Sketch {
    /// Build a `Sketch` using [`SketchBuilder`]
    pub fn builder(_: &Objects) -> SketchBuilder {
        SketchBuilder {
            surface: None,
            faces: FaceSet::new(),
        }
    }

    /// Construct an empty instance of `Sketch`
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Access the sketch's faces
    pub fn faces(&self) -> &FaceSet {
        &self.faces
    }
}
