use crate::{geometry::Handle, math::Vector};

use super::face::Face;

pub trait TranslateExt {
    fn translate(self, offset: impl Into<Vector<3>>) -> Translate;
}

impl TranslateExt for Handle<Face> {
    fn translate(self, offset: impl Into<Vector<3>>) -> Translate {
        let offset = offset.into();

        let output = Handle::new(Face::new(
            self.surface().translate(offset),
            self.vertices()
                .map(|vertex| Handle::new(vertex.translate(offset))),
        ));

        Translate { output }
    }
}

pub struct Translate {
    pub output: Handle<Face>,
}
