use std::collections::BTreeSet;

use super::Face;

/// A 2-dimensional shape
///
/// # Implementation Note
///
/// The faces that make up the sketch must be in the same surface. This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    faces: BTreeSet<Face>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new() -> Self {
        Self {
            faces: BTreeSet::new(),
        }
    }

    /// Construct a sketch from faces
    pub fn from_faces(
        faces: impl IntoIterator<Item = impl Into<Face>>,
    ) -> Self {
        let faces = faces.into_iter().map(Into::into).collect();
        Self { faces }
    }

    /// Access the sketch's faces
    pub fn faces(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter()
    }

    /// Convert the sketch into a list of faces
    pub fn into_faces(self) -> BTreeSet<Face> {
        self.faces
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}
