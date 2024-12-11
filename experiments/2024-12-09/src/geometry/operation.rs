use std::{fmt, rc::Rc};

use super::{Triangle, Vertex};

pub trait Operation: fmt::Display {
    fn vertices(&self, vertices: &mut Vec<Vertex>);
    fn triangles(&self, triangles: &mut Vec<Triangle>);
    fn children(&self) -> Vec<HandleAny>;
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

impl fmt::Display for HandleAny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Operation for HandleAny {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        self.inner.vertices(vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        self.inner.triangles(triangles);
    }

    fn children(&self) -> Vec<HandleAny> {
        self.inner.children()
    }
}
