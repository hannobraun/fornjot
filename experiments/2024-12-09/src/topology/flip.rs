use std::fmt;

use crate::geometry::{Handle, HandleAny, Operation, OperationOutput, TriMesh};

use super::face::Face;

pub trait FlipExt {
    fn flip(self) -> Flip;
}

impl<T> FlipExt for &T
where
    T: OperationOutput<Face>,
{
    fn flip(self) -> Flip {
        let output = Handle::new(Face::new(
            self.output().surface().flip(),
            self.output().vertices().cloned(),
        ));

        Flip { output }
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
