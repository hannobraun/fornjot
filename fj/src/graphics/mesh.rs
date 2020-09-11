use super::{Index, Vertex};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}
