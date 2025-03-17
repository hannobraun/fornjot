use std::fmt;

use crate::geometry::TriMesh;

use super::HandleAny;

/// # A trait that is implemented by all "objects", whatever those are
///
/// This trait is the problem child of this experiment. I wanted to use it to
/// create a much more detailed and interactive view of objects, but this ended
/// up as just a simple tree that is rendered next to the object.
///
/// It's probably safe to ignore most of the stuff here. My current plan is to
/// strip this down to its essentials, completely remove the object tree from
/// the debug view, and experiment with other means of providing visibility into
/// how shapes are structured and constructed.
pub trait Object {
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
    pub op: &'r dyn Object,
}

impl fmt::Display for OperationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.op.display(f)
    }
}
