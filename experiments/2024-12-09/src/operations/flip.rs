use crate::topology::{face::Face, surface::Surface};

pub trait FlipExt {
    fn flip(&self) -> Self;
}

impl FlipExt for Face {
    fn flip(&self) -> Self {
        Face::new(
            self.surface.flip(),
            self.half_edges.clone(),
            self.is_internal,
        )
    }
}

impl FlipExt for Surface {
    fn flip(&self) -> Self {
        let geometry = self.geometry.flip();
        Self { geometry }
    }
}
