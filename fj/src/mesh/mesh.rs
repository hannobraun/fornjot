use std::{collections::HashMap, convert::TryInto};

use decorum::R32;
use nalgebra::Point;

use crate::graphics;

pub struct Mesh {
    indices_by_vertex: HashMap<Vertex, graphics::Index>,

    vertices: Vec<Vertex>,
    triangles: Vec<[graphics::Index; 3]>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn triangle(
        &mut self,
        v0: impl Into<Point<f32, 3>>,
        v1: impl Into<Point<f32, 3>>,
        v2: impl Into<Point<f32, 3>>,
    ) {
        let v0 = v0.into();
        let v1 = v1.into();
        let v2 = v2.into();

        let v0 = Vertex {
            position: v0.map(|coord| coord.into()),
        };
        let v1 = Vertex {
            position: v1.map(|coord| coord.into()),
        };
        let v2 = Vertex {
            position: v2.map(|coord| coord.into()),
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

        self.triangles.push([i0, i1, i2]);
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vertex> + '_ {
        self.vertices.iter().copied()
    }

    pub fn indices(&self) -> impl Iterator<Item = graphics::Index> + '_ {
        self.triangles.iter().flatten().copied()
    }

    pub fn triangles(&self) -> impl Iterator<Item = [graphics::Index; 3]> + '_ {
        self.triangles.iter().copied()
    }

    fn index_for_vertex(&mut self, vertex: Vertex) -> graphics::Index {
        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Vertex {
    pub position: Point<R32, 3>,
}
