use std::convert::TryInto;

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

    pub fn vertex(&mut self, position: [f32; 3], normal: [f32; 3]) -> Index {
        let i = self.vertices.len();
        self.vertices.push(Vertex { position, normal });
        i.try_into().unwrap()
    }
}
