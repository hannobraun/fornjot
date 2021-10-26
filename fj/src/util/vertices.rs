use std::{collections::HashMap, convert::TryInto};

use decorum::R32;

use crate::{math::Point, types::Index};

#[derive(Clone)]
pub struct Vertices<T, const D: usize> {
    vertices: Vec<T>,
    indices_by_vertex: HashMap<Vertex<D>, Index>,
}

impl<T, const D: usize> Vertices<T, D> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices_by_vertex: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_
    where
        T: Copy,
    {
        self.vertices.iter().copied()
    }

    pub fn index_for_vertex(&mut self, vertex: T) -> Index
    where
        T: AsPoint<D>,
    {
        let vertex_r32 = vertex.as_point().map(|coord| coord.into());

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

pub trait AsPoint<const D: usize> {
    fn as_point(&self) -> Point<D>;
}

impl<const D: usize> AsPoint<D> for Point<D> {
    fn as_point(&self) -> Point<D> {
        *self
    }
}

type Vertex<const D: usize> = nalgebra::Point<R32, D>;
