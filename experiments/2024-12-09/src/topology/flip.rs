use crate::geometry::Handle;

use super::face::Face;

pub trait FlipExt {
    fn flip(self) -> Handle<Face>;
}

impl FlipExt for Handle<Face> {
    fn flip(self) -> Handle<Face> {
        Handle::new(Face::new(self.surface().flip(), self.vertices().cloned()))
    }
}
