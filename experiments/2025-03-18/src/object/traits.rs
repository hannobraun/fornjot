use std::fmt;

use crate::geometry::TriMesh;

use super::HandleAny;

pub trait Object {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn tri_mesh(&self) -> TriMesh;
    fn children(&self) -> Vec<HandleAny>;
}

pub struct OperationDisplay<'r> {
    pub op: &'r dyn Object,
}

impl fmt::Display for OperationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.op.display(f)
    }
}
