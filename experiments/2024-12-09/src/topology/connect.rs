use crate::geometry::Handle;

use super::{face::Face, solid::Solid};

pub trait ConnectExt {
    fn connect(self, other: Handle<Face>) -> Solid;
}

impl ConnectExt for Handle<Face> {
    fn connect(self, other: Handle<Face>) -> Solid {
        Solid::connect_faces([self, other])
    }
}
