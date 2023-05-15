use crate::{
    objects::{Face, Set},
    storage::Handle,
};

/// A 3-dimensional closed shell
///
/// # Implementation Note
///
/// The faces that make up a shell should be closed ("watertight"). This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Shell {
    faces: Set<Face>,
}

impl Shell {
    /// Construct an empty instance of `Shell`
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Access the faces of the shell
    pub fn faces(&self) -> &Set<Face> {
        &self.faces
    }

    /// Find the given face in the shell
    pub fn find_face(&self, face: &Handle<Face>) -> Option<Handle<Face>> {
        self.faces().find(face)
    }
}
