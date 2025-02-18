use std::fmt;

use crate::geometry::TriMesh;

use super::HandleAny;

pub trait Operation {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn tri_mesh(&self) -> TriMesh;
    fn children(&self) -> Vec<HandleAny>;

    fn label(&self) -> OperationDisplay
    where
        Self: Sized,
    {
        OperationDisplay { op: self as &_ }
    }
}

pub struct OperationDisplay<'r> {
    pub op: &'r dyn Operation,
}

impl fmt::Display for OperationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.op.display(f)
    }
}
