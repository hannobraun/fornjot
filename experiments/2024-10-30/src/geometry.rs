use tuples::CombinRight;

use crate::math::Point;

#[derive(Default)]
pub struct Operations {
    pub triangles: Vec<Triangle>,
    pub operations: Vec<OperationInSequence>,
}

impl Operations {
    pub fn vertex(
        &mut self,
        point: impl Into<Point>,
    ) -> OperationResult<(Vertex,)> {
        let vertex = Vertex {
            point: point.into(),
        };
        self.operations.push(OperationInSequence {
            operation: ClonedOperation::from_op(&vertex),
            previous: self
                .operations
                .last()
                .map(|op| ClonedOperation::from_op(op)),
        });

        OperationResult {
            operations: self,
            results: (vertex,),
        }
    }

    pub fn triangle(&mut self, triangle: Triangle) -> &mut Self {
        self.triangles.push(triangle);
        self
    }
}

impl Operation for Operations {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        if let Some(op) = self.operations.last() {
            op.vertices(vertices);
        }
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

    pub fn results(self) -> T {
        self.results
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

impl ClonedOperation {
    pub fn from_op(op: &dyn Operation) -> Self {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        op.vertices(&mut vertices);
        op.triangles(&mut triangles);

        Self {
            vertices,
            triangles,
        }
    }
}

impl Operation for ClonedOperation {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.extend(&self.vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        triangles.extend(&self.triangles);
    }
}
