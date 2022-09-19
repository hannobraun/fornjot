use crate::{builder::SketchBuilder, stores::Stores};

use super::{face::Faces, Face, Surface};

/// A 2-dimensional shape
///
/// # Implementation Note
///
/// The faces that make up the sketch must be in the same surface. This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    faces: Faces,
}

impl Sketch {
    /// Build a sketch using [`SketchBuilder`]
    pub fn build(stores: &Stores, surface: Surface) -> SketchBuilder {
        SketchBuilder::new(stores, surface)
    }

    /// Construct an empty instance of `Sketch`
    pub fn new() -> Self {
        Self {
            faces: Faces::new(),
        }
    }

    /// Add faces to the sketch
    ///
    /// Consumes the sketch and returns the updated instance.
    pub fn with_faces(
        mut self,
        faces: impl IntoIterator<Item = impl Into<Face>>,
    ) -> Self {
        let faces = faces.into_iter().map(Into::into);
        self.faces.extend(faces);
        self
    }

    /// Access the sketch's faces
    pub fn faces(&self) -> &Faces {
        &self.faces
    }

    /// Convert the sketch into a list of faces
    pub fn into_faces(self) -> Faces {
        self.faces
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}
