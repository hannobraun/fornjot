use crate::{
    handle::Handle,
    topology::{face::Face, surface::Surface},
};

pub trait Flip {
    fn flip(&self) -> Self;
}

impl Flip for Face {
    fn flip(&self) -> Self {
        Face::new(
            Handle::new(self.surface.flip()),
            self.half_edges.clone(),
            self.is_internal,
        )
    }
}

impl Flip for Surface {
    fn flip(&self) -> Self {
        let geometry = self.geometry.flip();
        Self { geometry }
    }
}
