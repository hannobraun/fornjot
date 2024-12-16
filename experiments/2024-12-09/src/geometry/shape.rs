use std::fmt;

use tuples::CombinRight;

use super::{
    operation::{Handle, HandleAny},
    Operation, Triangle, Vertex,
};

#[derive(Default)]
pub struct Shape {
    operations: Vec<OperationInSequence>,
}

impl Shape {
    pub fn extend(&mut self) -> OperationResult<()> {
        OperationResult {
            operations: &mut self.operations,
            results: (),
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "shape")
    }
}

impl Operation for Shape {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        if let Some(op) = self.operations.last() {
            op.vertices(vertices);
        }
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        if let Some(op) = self.operations.last() {
            op.triangles(triangles);
        }
    }

    fn children(&self) -> Vec<HandleAny> {
        self.operations
            .iter()
            .map(|op| HandleAny::new(op.clone()))
            .collect()
    }
}

#[derive(Clone)]
struct OperationInSequence {
    pub operation: HandleAny,
    pub previous: Option<HandleAny>,
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

    fn children(&self) -> Vec<HandleAny> {
        self.operation.children()
    }
}

impl fmt::Display for OperationInSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.operation.fmt(f)
    }
}

pub struct OperationResult<'r, T> {
    operations: &'r mut Vec<OperationInSequence>,
    results: T,
}

impl<'r, T> OperationResult<'r, T> {
    pub fn vertex(
        self,
        vertex: impl Into<Vertex>,
    ) -> OperationResult<'r, T::Out>
    where
        T: CombinRight<Handle<Vertex>>,
    {
        let vertex = Handle::new(vertex.into());

        self.operations.push(OperationInSequence {
            operation: vertex.to_any(),
            previous: self
                .operations
                .last()
                .map(|op| HandleAny::new(op.clone())),
        });

        OperationResult {
            operations: self.operations,
            results: self.results.push_right(vertex),
        }
    }

    pub fn triangle(
        self,
        triangle: impl Into<Triangle>,
    ) -> OperationResult<'r, T::Out>
    where
        T: CombinRight<Handle<Triangle>>,
    {
        let triangle = Handle::new(triangle.into());

        self.operations.push(OperationInSequence {
            operation: triangle.to_any(),
            previous: self
                .operations
                .last()
                .map(|op| HandleAny::new(op.clone())),
        });

        OperationResult {
            operations: self.operations,
            results: self.results.push_right(triangle),
        }
    }

    pub fn results(self) -> T {
        self.results
    }
}
