use crate::geometry::Handle;

use super::{face::Face, solid::Solid};

pub trait ConnectExt {
    /// # Connect two faces by creating a side wall of faces from their vertices
    ///
    /// ## Panics
    ///
    /// Panics, if the two faces provided do not have the same number of
    /// vertices.
    ///
    /// ## Implementation Note
    ///
    /// This method has very particular (and undocumented) requirements about
    /// the orientation of the two faces relative to each other, and will
    /// happily generate invalid geometry, if those undocumented requirements
    /// aren't met.
    ///
    /// It should be seen as more of a placeholder for a real implementation of
    /// this operation.
    fn connect(self, other: Handle<Face>) -> Solid;
}

impl ConnectExt for Handle<Face> {
    fn connect(self, other: Handle<Face>) -> Solid {
        Solid::connect_faces([self, other])
    }
}
