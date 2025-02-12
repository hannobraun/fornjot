use crate::geometry::Handle;

use super::face::Face;

pub trait FlipExt {
    fn flip(self) -> Flip;
}

impl FlipExt for Handle<Face> {
    fn flip(self) -> Flip {
        let output = Handle::new(Face::new(
            self.surface().flip(),
            self.vertices().cloned(),
        ));

        Flip { output }
    }
}

pub struct Flip {
    pub output: Handle<Face>,
}
