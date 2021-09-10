use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::Point;

use crate::types::Index;

pub struct Vertices {
    vertices: Vec<Point<f32, 3>>,
    indices_by_vertex: HashMap<Vertex, Index>,
}

impl Vertices {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices_by_vertex: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Point<f32, 3>> + '_ {
        self.vertices.iter().copied()
    }

    pub fn index_for_vertex(&mut self, vertex: impl AsPoint) -> Index {
        let vertex = vertex.as_point();
        let vertex_r32 = vertex.map(|coord| coord.into());

        let vertices = &mut self.vertices;

        let index =
            self.indices_by_vertex.entry(vertex_r32).or_insert_with(|| {
                let index = vertices.len();
                vertices.push(vertex);
                index.try_into().unwrap()
            });

        *index
    }
}

pub trait AsPoint {
    fn as_point(&self) -> Point<f32, 3>;
}

impl AsPoint for Point<f32, 3> {
    fn as_point(&self) -> Point<f32, 3> {
        *self
    }
}

type Vertex = Point<R32, 3>;
