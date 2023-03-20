use crate::{
    objects::{Face, Set},
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
    faces: Set<Face>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Access the faces of the sketch
    pub fn faces(&self) -> &Set<Face> {
        &self.faces
    }
}
