use crate::{
    math::Vector,
    operation::{Handle, OperationOutput},
};

use super::face::Face;

pub trait TranslateExt {
    fn translate(self, offset: impl Into<Vector<3>>) -> Handle<Face>;
}

impl TranslateExt for &Face {
    fn translate(self, offset: impl Into<Vector<3>>) -> Handle<Face> {
        let offset = offset.into();

        Handle::new(Face::new(
            self.output().surface().translate(offset),
            self.output()
                .vertices()
                .map(|vertex| Handle::new(vertex.output().translate(offset))),
        ))
    }
}
