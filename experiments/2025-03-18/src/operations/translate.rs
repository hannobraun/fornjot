use fj_math::Vector;

use crate::{
    handle::Handle,
    topology::{
        curve::Curve, face::Face, half_edge::HalfEdge, surface::Surface,
        vertex::Vertex,
    },
};

pub trait TranslateExt {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self;
}

impl TranslateExt for Curve {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let geometry = self.geometry.translate(offset);
        Curve { geometry }
    }
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
        let offset = offset.into();

        let curve = self.curve.translate(offset);
        let start = self.start.translate(offset);

        HalfEdge {
            curve: Handle::new(curve),
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
