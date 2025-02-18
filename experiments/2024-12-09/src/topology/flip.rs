use std::fmt;

use crate::{
    geometry::TriMesh,
    operation::{Handle, HandleAny, Operation, OperationOutput},
    topology::face::Face,
};

pub trait FlipExt {
    fn flip(self) -> Handle<Face>;
}

impl FlipExt for &Face {
    fn flip(self) -> Handle<Face> {
        Handle::new(Face::new(self.surface().flip(), self.vertices().cloned()))
    }
}

pub struct Flip {
    pub output: Handle<Face>,
}

impl Operation for Flip {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flip")
    }

    fn tri_mesh(&self) -> TriMesh {
        self.output.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        vec![self.output.to_any()]
    }
}

impl OperationOutput<Face> for Flip {
    fn output(&self) -> &Face {
        self.output.output()
    }
}
