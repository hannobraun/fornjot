use std::rc::Rc;

use crate::geometry::TriMesh;

use super::Object;

#[derive(Clone)]
pub struct HandleAny {
    pub(super) inner: Rc<dyn Object>,
}

impl HandleAny {
    pub fn new(op: impl Object + 'static) -> Self {
        Self { inner: Rc::new(op) }
    }
}

impl Object for HandleAny {
    fn tri_mesh(&self) -> TriMesh {
        self.inner.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        self.inner.children()
    }
}
