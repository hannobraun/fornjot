use super::Face;

/// A 3-dimensional shape
///
/// # Implementation Note
///
/// The faces that make up the solid must form a closed shape. This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Solid {
    faces: Vec<Face>,
}

impl Solid {
    /// Construct a solid from faces
    pub fn from_faces(faces: impl IntoIterator<Item = Face>) -> Self {
        let faces = faces.into_iter().collect();
        Self { faces }
    }

    /// Access the solid's faces
    pub fn faces(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter()
    }

    /// Convert the solid into a list of faces
    pub fn into_faces(self) -> Vec<Face> {
        self.faces
    }
}
