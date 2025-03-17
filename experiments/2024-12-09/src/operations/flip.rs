use crate::{
    object::Handle,
    topology::{face::Face, surface::Surface},
};

/// # Extension trait for objects that can be flipped
pub trait FlipExt {
    /// # Flip a face or surface
    ///
    /// This might be subsumed by a more general "transform" operation later.
    /// Not sure!
    fn flip(&self) -> Self;
}

impl FlipExt for Face {
    fn flip(&self) -> Self {
        Face::new(
            Handle::new(self.surface.flip()),
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
