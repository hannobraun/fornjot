use crate::{operation::Handle, topology::face::Face};

pub trait FlipExt {
    fn flip(self) -> Handle<Face>;
}

impl FlipExt for &Face {
    fn flip(self) -> Handle<Face> {
        Handle::new(Face::new(self.surface().flip(), self.vertices().cloned()))
    }
}
