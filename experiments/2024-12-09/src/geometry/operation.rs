use std::{cmp::Ordering, fmt, ops::Deref, rc::Rc};

use super::tri_mesh::TriMesh;

pub trait Operation {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn tri_mesh(&self) -> TriMesh;
    fn children(&self) -> Vec<AnyOp>;

    fn label(&self) -> OperationDisplay
    where
        Self: Sized,
    {
        OperationDisplay { op: self as &_ }
    }
}

pub trait OperationOutput<Output = Self>: Operation {
    fn output(&self) -> &Output;
}

pub struct OperationDisplay<'r> {
    pub op: &'r dyn Operation,
}

impl fmt::Display for OperationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.op.display(f)
    }
}

pub struct Handle<T> {
    inner: Rc<T>,
}

impl<T> Handle<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    pub fn to_any(&self) -> AnyOp
    where
        T: Operation + 'static,
    {
        self.clone().into_any()
    }

    pub fn into_any(self) -> AnyOp
    where
        T: Operation + 'static,
    {
        AnyOp { inner: self.inner }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.as_ref()
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

#[derive(Clone)]
pub struct AnyOp {
    inner: Rc<dyn Operation>,
}

impl AnyOp {
    pub fn new(op: impl Operation + 'static) -> Self {
        Self { inner: Rc::new(op) }
    }
}

impl Operation for AnyOp {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.display(f)?;
        write!(f, " ({:?})", Rc::as_ptr(&self.inner))?;

        Ok(())
    }

    fn tri_mesh(&self) -> TriMesh {
        self.inner.tri_mesh()
    }

    fn children(&self) -> Vec<AnyOp> {
        self.inner.children()
    }
}

impl OperationOutput for AnyOp {
    fn output(&self) -> &Self {
        self
    }
}
