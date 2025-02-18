use std::fmt;

use crate::{
    geometry::TriMesh,
    math::Vector,
    operation::{Handle, HandleAny, Operation, OperationOutput},
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

pub struct Translate {
    pub output: Handle<Face>,
}

impl Operation for Translate {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Translate")
    }

    fn tri_mesh(&self) -> TriMesh {
        self.output.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        vec![self.output.to_any()]
    }
}

impl OperationOutput<Face> for Translate {
    fn output(&self) -> &Face {
        self.output.output()
    }
}
