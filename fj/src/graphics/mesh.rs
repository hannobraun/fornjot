use std::convert::TryInto;

use decorum::R32;

use super::{Index, Vertex};

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
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
        self.vertices.push(Vertex {
            position: [
                R32::from_inner(position[0]),
                R32::from_inner(position[1]),
                R32::from_inner(position[2]),
            ],
            normal: [
                R32::from_inner(normal[0]),
                R32::from_inner(normal[1]),
                R32::from_inner(normal[2]),
            ],
        });
        i.try_into().unwrap()
    }

    pub fn triangle(&mut self, i0: Index, i1: Index, i2: Index) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[Index] {
        self.indices.as_slice()
    }
}
