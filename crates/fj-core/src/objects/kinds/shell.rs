use crate::{
    objects::{handles::Handles, Face, HandleIter},
    storage::Handle,
};

/// A 3-dimensional closed shell
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Shell {
    faces: Handles<Face>,
}

impl Shell {
    /// Construct an empty instance of `Shell`
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Access the faces of the shell
    pub fn faces(&self) -> HandleIter<Face> {
        self.faces.iter()
    }
}
