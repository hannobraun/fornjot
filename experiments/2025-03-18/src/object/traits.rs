use std::fmt;

use crate::geometry::TriMesh;

use super::HandleAny;

pub trait Object {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn tri_mesh(&self) -> TriMesh;
    fn children(&self) -> Vec<HandleAny>;
}
