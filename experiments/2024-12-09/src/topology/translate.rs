use crate::{geometry::Handle, math::Vector};

use super::face::Face;

pub trait TranslateExt {
    fn translate(self, offset: impl Into<Vector<3>>) -> Face;
}

impl TranslateExt for Handle<Face> {
    fn translate(self, offset: impl Into<Vector<3>>) -> Face {
        let offset = offset.into();

        Face::new(
            self.surface().translate(offset),
            self.vertices()
                .map(|vertex| Handle::new(vertex.translate(offset))),
        )
    }
}
