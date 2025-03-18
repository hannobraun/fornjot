use crate::geometry::TriMesh;

use super::HandleAny;

pub trait Object {
    fn tri_mesh(&self) -> TriMesh;
    fn children(&self) -> Vec<HandleAny>;
}
