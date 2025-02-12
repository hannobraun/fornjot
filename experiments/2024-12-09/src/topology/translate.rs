use std::ops::Deref;

use crate::{geometry::Handle, math::Vector};

use super::face::Face;

pub trait TranslateExt {
    fn translate(self, offset: impl Into<Vector<3>>) -> Face;
}

impl TranslateExt for Handle<Face> {
    fn translate(self, offset: impl Into<Vector<3>>) -> Face {
        self.deref().translate(offset)
    }
}
