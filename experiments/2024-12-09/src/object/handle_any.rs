use std::{fmt, rc::Rc};

use crate::geometry::TriMesh;

use super::Object;

/// # An untyped handle that can be used to abstract over objects
///
/// Can be used wherever you need to iterate over objects of various types.
///
/// See documentation of `Handle` for more context on handles and object
/// storage.
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
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.display(f)?;
        write!(f, " ({:?})", Rc::as_ptr(&self.inner))?;

        Ok(())
    }

    fn tri_mesh(&self) -> TriMesh {
        self.inner.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        self.inner.children()
    }
}
