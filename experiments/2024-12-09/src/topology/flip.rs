use crate::geometry::Handle;

use super::face::Face;

pub trait FlipExt {
    fn flip(self) -> Face;
}

impl FlipExt for Handle<Face> {
    fn flip(self) -> Face {
        Face::new(self.surface().flip(), self.vertices().cloned())
    }
}
