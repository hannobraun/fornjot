use crate::math::Point;

#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn vertex(&mut self, point: Point) {
        let vertex = Vertex { point };
        self.vertices.push(vertex);
    }

    pub fn triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

impl Operation for Mesh {
    fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    fn triangles(&self) -> Vec<Triangle> {
        self.triangles.clone()
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub point: Point,
}

impl Operation for Vertex {
    fn vertices(&self) -> Vec<Vertex> {
        vec![*self]
    }

    fn triangles(&self) -> Vec<Triangle> {
        vec![]
    }
}

pub type Index = u32;
pub type Triangle = [Index; 3];

pub trait Operation {
    fn vertices(&self) -> Vec<Vertex>;
    fn triangles(&self) -> Vec<Triangle>;
}

pub struct OperationInSequence {
    pub operation: Box<dyn Operation>,
    pub previous: Option<Box<dyn Operation>>,
}

impl Operation for OperationInSequence {
    fn vertices(&self) -> Vec<Vertex> {
        let mut vertices = self
            .previous
            .as_ref()
            .map(|op| op.vertices())
            .unwrap_or_default();
        vertices.extend(self.operation.vertices());

        vertices
    }

    fn triangles(&self) -> Vec<Triangle> {
        let mut triangles = self
            .previous
            .as_ref()
            .map(|op| op.triangles())
            .unwrap_or_default();
        triangles.extend(self.operation.triangles());

        triangles
    }
}
