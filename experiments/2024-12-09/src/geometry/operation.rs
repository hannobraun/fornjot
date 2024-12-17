use std::{fmt, rc::Rc};

use super::{Triangle, Vertex};

pub trait Operation: fmt::Display {
    fn vertices(&self, vertices: &mut Vec<Vertex>);
    fn triangles(&self, triangles: &mut Vec<Triangle>);
    fn children(&self) -> Vec<AnyOp>;
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Handle<T> {
    inner: Rc<T>,
}

impl<T> Handle<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        *self.as_ref()
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

impl<T> AsRef<T> for Handle<T> {
    fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
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

impl fmt::Display for AnyOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Operation for AnyOp {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        self.inner.vertices(vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        self.inner.triangles(triangles);
    }

    fn children(&self) -> Vec<AnyOp> {
        self.inner.children()
    }
}
