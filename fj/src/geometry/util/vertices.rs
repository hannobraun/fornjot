use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::Point;

use crate::graphics::Index;

#[derive(Default)]
pub struct Vertices {
    indices_by_vertex: HashMap<Vertex, Index>,
    vertices: Vec<Vertex>,
}

impl Vertices {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = Point<f32, 3>> + '_ {
        self.vertices
            .iter()
            .copied()
            .map(|v| v.map(|coord| coord.into()))
    }

    pub fn index_for_vertex(&mut self, vertex: Point<f32, 3>) -> Index {
        let vertex = vertex.map(|coord| coord.into());

        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}

type Vertex = Point<R32, 3>;
