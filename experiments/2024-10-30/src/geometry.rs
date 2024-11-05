use crate::math::Point;

#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn vertices(&self) -> impl Iterator<Item = Vertex> + '_ {
        self.vertices.iter().copied()
    }

    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.triangles.iter().copied()
    }

    pub fn vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub point: Point,
}

pub type Index = u32;
pub type Triangle = [Index; 3];
