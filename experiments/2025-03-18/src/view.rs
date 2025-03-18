use crate::{
    geometry::TriMesh,
    object::{HandleAny, Object},
};

#[derive(Clone)]
pub struct OperationView {
    operation: HandleAny,
}

impl Object for OperationView {
    fn tri_mesh(&self) -> TriMesh {
        self.operation.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        self.operation.children()
    }
}
