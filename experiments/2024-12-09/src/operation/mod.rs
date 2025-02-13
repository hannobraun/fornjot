mod handle;
mod traits;

pub use self::{
    handle::Handle,
    traits::{Operation, OperationOutput},
};

use std::{fmt, rc::Rc};

use crate::geometry::TriMesh;

#[derive(Clone)]
pub struct HandleAny {
    inner: Rc<dyn Operation>,
}

impl HandleAny {
    pub fn new(op: impl Operation + 'static) -> Self {
        Self { inner: Rc::new(op) }
    }
}

impl Operation for HandleAny {
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

impl OperationOutput for HandleAny {
    fn output(&self) -> &Self {
        self
    }
}
