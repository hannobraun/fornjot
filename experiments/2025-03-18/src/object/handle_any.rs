use std::rc::Rc;

use crate::geometry::TriMesh;

use super::Object;

#[derive(Clone)]
pub struct HandleAny {
    pub(super) inner: Rc<dyn Object>,
}

impl Object for HandleAny {
    fn tri_mesh(&self) -> TriMesh {
        self.inner.tri_mesh()
    }
}
