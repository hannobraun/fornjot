use crate::{
    math::Vector,
    object::Handle,
    topology::{face::Face, half_edge::HalfEdge, vertex::Vertex},
};

pub trait TranslateExt {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self;
}

impl TranslateExt for Face {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Face::new(
            self.surface.geometry.translate(offset),
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

impl TranslateExt for Vertex {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();
        Vertex::new(self.point + offset)
    }
}
