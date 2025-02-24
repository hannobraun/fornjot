use crate::{math::Vector, object::Handle, topology::face::Face};

pub trait TranslateExt {
    fn translate(self, offset: impl Into<Vector<3>>) -> Self;
}

impl TranslateExt for Face {
    fn translate(self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Face::new(
            self.surface().translate(offset),
            self.half_edges().map(|half_edge| {
                Handle::new(half_edge.start().translate(offset))
            }),
        )
    }
}
