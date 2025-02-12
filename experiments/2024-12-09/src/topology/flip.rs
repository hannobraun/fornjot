use std::ops::Deref;

use crate::geometry::Handle;

use super::face::Face;

pub trait FlipExt {
    fn flip(self) -> Face;
}

impl FlipExt for Handle<Face> {
    fn flip(self) -> Face {
        self.deref().flip()
    }
}
