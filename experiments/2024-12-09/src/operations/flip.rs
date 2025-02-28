use crate::topology::face::Face;

pub trait FlipExt {
    fn flip(self) -> Face;
}

impl FlipExt for &Face {
    fn flip(self) -> Face {
        Face::new(
            self.surface.geometry.flip(),
            self.half_edges.clone(),
            self.is_internal,
        )
    }
}
