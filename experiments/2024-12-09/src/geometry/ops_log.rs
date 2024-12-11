use std::{fmt, rc::Rc};

use tuples::CombinRight;

use super::{operation::AnyOp, Operation, Triangle, Vertex};

#[derive(Default)]
pub struct OpsLog {
    operations: Vec<OperationInSequence>,
}

impl OpsLog {
    pub fn vertex(
        &mut self,
        vertex: impl Into<Vertex>,
    ) -> OperationResult<(Vertex,)> {
        let vertex = vertex.into();

        self.operations.push(OperationInSequence {
            operation: AnyOp::new(vertex),
            previous: self.operations.last().map(|op| Rc::new(op.clone()) as _),
        });

        OperationResult {
            operations: self,
            results: (vertex,),
        }
    }

    pub fn triangle(
        &mut self,
        triangle: impl Into<Triangle>,
    ) -> OperationResult<(Triangle,)> {
        let triangle = triangle.into();

        self.operations.push(OperationInSequence {
            operation: AnyOp::new(triangle),
            previous: self.operations.last().map(|op| Rc::new(op.clone()) as _),
        });

        OperationResult {
            operations: self,
            results: (triangle,),
        }
    }
}

impl fmt::Display for OpsLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(op) = self.operations.last() {
            op.fmt(f)
        } else {
            write!(f, "empty operations log")
        }
    }
}

impl Operation for OpsLog {
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

    fn children(&self) -> Vec<AnyOp> {
        self.operations
            .iter()
            .map(|op| AnyOp::new(op.clone()))
            .collect()
    }
}

#[derive(Clone)]
struct OperationInSequence {
    pub operation: AnyOp,
    pub previous: Option<Rc<dyn Operation>>,
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

    fn children(&self) -> Vec<AnyOp> {
        self.operation.children()
    }
}

impl fmt::Display for OperationInSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.operation.fmt(f)
    }
}

pub struct OperationResult<'r, T> {
    operations: &'r mut OpsLog,
    results: T,
}

impl<'r, T> OperationResult<'r, T> {
    pub fn vertex(
        self,
        vertex: impl Into<Vertex>,
    ) -> OperationResult<'r, T::Out>
    where
        T: CombinRight<Vertex>,
    {
        let OperationResult {
            results: (vertex,), ..
        } = self.operations.vertex(vertex);

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
        T: CombinRight<Triangle>,
    {
        let OperationResult {
            results: (triangle,),
            ..
        } = self.operations.triangle(triangle);

        OperationResult {
            operations: self.operations,
            results: self.results.push_right(triangle),
        }
    }

    pub fn results(self) -> T {
        self.results
    }
}
