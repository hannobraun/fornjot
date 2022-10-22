use crate::{builder::ShellBuilder, storage::Handle};

use super::{face::Faces, Face, Objects};

/// A 3-dimensional closed shell
///
/// # Implementation Note
///
/// The faces that make up a shell should be closed ("watertight"). This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Shell {
    faces: Faces,
}

impl Shell {
    /// Build a `Shell` using [`ShellBuilder`]
    pub fn builder(objects: &Objects) -> ShellBuilder {
        ShellBuilder { objects }
    }

    /// Construct an empty instance of `Shell`
    pub fn new() -> Self {
        Self {
            faces: Faces::new(),
        }
    }

    /// Add faces to the shell
    ///
    /// Consumes the shell and returns the updated instance.
    pub fn with_faces(
        mut self,
        faces: impl IntoIterator<Item = Handle<Face>>,
    ) -> Self {
        let faces = faces.into_iter().map(Into::into);
        self.faces.extend(faces);
        self
    }

    /// Access the shell's faces
    pub fn faces(&self) -> &Faces {
        &self.faces
    }

    /// Convert the shell into a list of faces
    pub fn into_faces(self) -> Faces {
        self.faces
    }

    /// Find the given face in this shell
    pub fn find_face(&self, face: &Handle<Face>) -> Option<Handle<Face>> {
        self.faces().find(face)
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}
