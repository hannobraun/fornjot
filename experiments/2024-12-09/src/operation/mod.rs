mod traits;

pub use self::traits::{Operation, OperationOutput};

use std::{cmp::Ordering, fmt, rc::Rc};

use crate::geometry::TriMesh;

pub struct Handle<T> {
    inner: Rc<dyn OperationOutput<T>>,
}

impl<T> Handle<T> {
    pub fn new(inner: impl OperationOutput<T> + 'static) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    pub fn to_any(&self) -> HandleAny {
        self.clone().into_any()
    }

    pub fn into_any(self) -> HandleAny {
        HandleAny { inner: self.inner }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Rc::as_ptr(&self.inner)
            .cast::<()>()
            .cmp(&Rc::as_ptr(&other.inner).cast::<()>())
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Handle")
            .field("inner", &Rc::as_ptr(&self.inner))
            .finish()
    }
}

impl<T> Operation for Handle<T> {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.display(f)
    }

    fn tri_mesh(&self) -> TriMesh {
        self.inner.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        self.inner.children()
    }
}

impl<T> OperationOutput<T> for Handle<T> {
    fn output(&self) -> &T {
        self.inner.output()
    }
}

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
