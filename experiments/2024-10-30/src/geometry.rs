use crate::math::Point;

#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn vertex(&mut self, point: Point) {
        self.vertices.push(Vertex { point });
    }

    pub fn triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

impl Operation for Mesh {
    fn vertices(&self) -> impl Iterator<Item = Vertex> {
        self.vertices.iter().copied()
    }

    fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.triangles.iter().copied()
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub point: Point,
}

impl Operation for Vertex {
    fn vertices(&self) -> impl Iterator<Item = Vertex> {
        [*self].into_iter()
    }

    fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        [].into_iter()
    }
}

pub type Index = u32;
pub type Triangle = [Index; 3];

pub trait Operation {
    fn vertices(&self) -> impl Iterator<Item = Vertex>;
    fn triangles(&self) -> impl Iterator<Item = Triangle> + '_;
}
