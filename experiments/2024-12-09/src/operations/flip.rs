use crate::topology::face::Face;

pub trait FlipExt {
    fn flip(self) -> Face;
}

impl FlipExt for &Face {
    fn flip(self) -> Face {
        Face::new(self.surface.flip(), self.half_edges().cloned())
    }
}
