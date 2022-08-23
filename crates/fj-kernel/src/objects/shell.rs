use std::collections::BTreeSet;

use crate::builder::ShellBuilder;

use super::Face;

/// A 3-dimensional closed shell
///
/// # Implementation Note
///
/// The faces that make up a shell should be closed ("watertight"). This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Shell {
    faces: BTreeSet<Face>,
}

impl Shell {
    /// Build a shell using [`ShellBuilder`]
    pub fn build() -> ShellBuilder {
        ShellBuilder
    }

    /// Construct an empty instance of `Shell`
    pub fn new() -> Self {
        Self {
            faces: BTreeSet::new(),
        }
    }

    /// Add faces to the shell
    ///
    /// Consumes the shell and returns the updated instance.
    pub fn with_faces(
        mut self,
        faces: impl IntoIterator<Item = impl Into<Face>>,
    ) -> Self {
        let faces = faces.into_iter().map(Into::into);
        self.faces.extend(faces);
        self
    }

    /// Access the shell's faces
    pub fn faces(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter()
    }

    /// Convert the shell into a list of faces
    pub fn into_faces(self) -> impl Iterator<Item = Face> {
        self.faces.into_iter()
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}
