use tuples::CombinRight;

use crate::math::Point;

#[derive(Default)]
pub struct Operations {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Operations {
    pub fn vertex(
        &mut self,
        point: impl Into<Point>,
    ) -> OperationResult<(Vertex,)> {
        let vertex = Vertex {
            point: point.into(),
        };
        self.vertices.push(vertex);

        OperationResult {
            operations: self,
            results: (vertex,),
        }
    }

    pub fn triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

impl Operation for Operations {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.extend(&self.vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        triangles.extend(&self.triangles);
    }
}

pub struct OperationResult<'r, T> {
    operations: &'r mut Operations,
    results: T,
}

impl<'r, T> OperationResult<'r, T> {
    pub fn vertex(self, point: impl Into<Point>) -> OperationResult<'r, T::Out>
    where
        T: CombinRight<Vertex>,
    {
        let OperationResult {
            results: (vertex,), ..
        } = self.operations.vertex(point);

        OperationResult {
            operations: self.operations,
            results: self.results.push_right(vertex),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point,
}

impl Operation for Vertex {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.push(*self);
    }

    fn triangles(&self, _: &mut Vec<Triangle>) {}
}

pub type Triangle = [Vertex; 3];

pub trait Operation {
    fn vertices(&self, vertices: &mut Vec<Vertex>);
    fn triangles(&self, triangles: &mut Vec<Triangle>);
}

pub struct OperationInSequence {
    pub operation: ClonedOperation,
    pub previous: Option<ClonedOperation>,
}

impl Operation for OperationInSequence {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        if let Some(op) = &self.previous {
            op.vertices(vertices);
        }
        self.operation.vertices(vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        if let Some(op) = &self.previous {
            op.triangles(triangles);
        }
        self.operation.triangles(triangles);
    }
}

pub struct ClonedOperation {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Operation for ClonedOperation {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.extend(&self.vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        triangles.extend(&self.triangles);
    }
}
