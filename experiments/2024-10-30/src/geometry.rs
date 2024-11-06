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
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.extend(self.vertices.iter().copied());
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
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.push(*self);
    }

    fn triangles(&self) -> Vec<Triangle> {
        vec![]
    }
}

pub type Index = u32;
pub type Triangle = [Index; 3];

pub trait Operation {
    fn vertices(&self, vertices: &mut Vec<Vertex>);
    fn triangles(&self) -> Vec<Triangle>;
}

pub struct OperationInSequence {
    pub operation: ClonedOperation,
    pub previous: Option<Box<ClonedOperation>>,
}

impl Operation for OperationInSequence {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        if let Some(op) = &self.previous {
            op.vertices(vertices);
        }
        self.operation.vertices(vertices);
    }

    fn triangles(&self) -> Vec<Triangle> {
        let mut triangles = self
            .previous
            .as_ref()
            .map(|op| op.triangles.clone())
            .unwrap_or_default();
        triangles.extend(self.operation.triangles.clone());

        triangles
    }
}

pub struct ClonedOperation {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Operation for ClonedOperation {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.extend(self.vertices.iter());
    }

    fn triangles(&self) -> Vec<Triangle> {
        self.triangles.clone()
    }
}
