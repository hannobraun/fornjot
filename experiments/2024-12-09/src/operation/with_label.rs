use std::fmt;

use crate::geometry::TriMesh;

use super::{HandleAny, Operation, OperationOutput};

pub struct WithLabel<T> {
    pub text: &'static str,
    pub op: T,
}

impl<T> Operation for WithLabel<T>
where
    T: Operation,
{
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.text)?;
        self.op.display(f)
    }

    fn tri_mesh(&self) -> TriMesh {
        self.op.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        self.op.children()
    }
}

impl<T> OperationOutput<T> for WithLabel<T>
where
    T: OperationOutput<T>,
{
    fn output(&self) -> &T {
        self.op.output()
    }
}
