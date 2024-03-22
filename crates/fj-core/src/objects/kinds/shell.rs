use crate::{
    objects::{Face, ObjectSet},
    storage::Handle,
};

/// A 3-dimensional closed shell
#[derive(Clone, Debug)]
pub struct Shell {
    faces: ObjectSet<Face>,
}

impl Shell {
    /// Construct an empty instance of `Shell`
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Access the faces of the shell
    pub fn faces(&self) -> &ObjectSet<Face> {
        &self.faces
    }
}
