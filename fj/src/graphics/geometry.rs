use super::{Index, Vertex};

pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}
