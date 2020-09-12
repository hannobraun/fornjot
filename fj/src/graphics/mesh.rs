use std::{collections::HashMap, convert::TryInto};

use euclid::default::Point3D;

use decorum::R32;

use super::vertices::{Array3, Index, Vertex};

pub struct Mesh {
    positions: Vec<Array3>,
    indices_by_vertex: HashMap<Vertex, Index>,

    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            indices_by_vertex: HashMap::new(),

            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn vertex(&mut self, vertex: [f32; 3]) -> I {
        let i = self.positions.len();
        self.positions.push(Array3([
            R32::from_inner(vertex[0]),
            R32::from_inner(vertex[1]),
            R32::from_inner(vertex[2]),
        ]));
        I(i)
    }

    pub fn triangle(&mut self, i0: I, i1: I, i2: I) {
        let p0 = self.positions[i0.0];
        let p1 = self.positions[i1.0];
        let p2 = self.positions[i2.0];

        let normal = (Point3D::from(p1.0) - Point3D::from(p0.0))
            .cross(Point3D::from(p2.0) - Point3D::from(p0.0))
            .to_array();

        let v0 = Vertex {
            position: p0,
            normal: Array3(normal),
        };
        let v1 = Vertex {
            position: p1,
            normal: Array3(normal),
        };
        let v2 = Vertex {
            position: p2,
            normal: Array3(normal),
        };

        let i0 = self.index_for_vertex(v0);
        let i1 = self.index_for_vertex(v1);
        let i2 = self.index_for_vertex(v2);

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

    fn index_for_vertex(&mut self, vertex: Vertex) -> Index {
        let vertices = &mut self.vertices;

        let index = self.indices_by_vertex.entry(vertex).or_insert_with(|| {
            let index = vertices.len();
            vertices.push(vertex);
            index.try_into().unwrap()
        });

        *index
    }
}

#[derive(Clone, Copy)]
pub struct I(usize);
