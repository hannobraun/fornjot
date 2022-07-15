use super::Face;

/// A 2-dimensional shape
///
/// # Implementation Note
///
/// The faces that make up the sketch must be in the same surface. This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    faces: Vec<Face>,
}

impl Sketch {
    /// Construct a sketch from faces
    pub fn from_faces(faces: impl IntoIterator<Item = Face>) -> Self {
        let faces = faces.into_iter().collect();
        Self { faces }
    }

    /// Access the sketch's faces
    pub fn faces(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter()
    }

    /// Convert the sketch into a list of faces
    pub fn into_faces(self) -> Vec<Face> {
        self.faces
    }
}
