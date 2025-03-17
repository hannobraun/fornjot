use crate::{
    math::Vector,
    object::Handle,
    topology::{
        face::Face, half_edge::HalfEdge, surface::Surface, vertex::Vertex,
    },
};

/// # Extension trait for things that can be translated
///
/// This is the most versatile operation right now, as it's implemented for many
/// different types of objects.
///
/// I expect this to morph into a more general "transform" operation over time.
pub trait TranslateExt {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self;
}

impl TranslateExt for Face {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Face::new(
            Handle::new(self.surface.translate(offset)),
            self.half_edges
                .iter()
                .map(|half_edge| Handle::new(half_edge.translate(offset))),
            self.is_internal,
        )
    }
}

impl TranslateExt for HalfEdge {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let start = self.start.translate(offset);

        HalfEdge {
            start: Handle::new(start),
            is_internal: self.is_internal,
        }
    }
}

impl TranslateExt for Surface {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();
        let geometry = self.geometry.translate(offset);
        Self { geometry }
    }
}

impl TranslateExt for Vertex {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();
        Vertex::new(self.point + offset)
    }
}
