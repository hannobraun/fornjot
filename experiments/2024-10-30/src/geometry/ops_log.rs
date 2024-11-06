use tuples::CombinRight;

use super::{Operation, Triangle, Vertex};

#[derive(Default)]
pub struct OpsLog {
    pub triangles: Vec<Triangle>,
    pub operations: Vec<OperationInSequence>,
}

impl OpsLog {
    pub fn vertex(
        &mut self,
        vertex: impl Into<Vertex>,
    ) -> OperationResult<(Vertex,)> {
        let vertex = vertex.into();

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

    pub fn triangle(
        &mut self,
        triangle: Triangle,
    ) -> OperationResult<(Triangle,)> {
        self.triangles.push(triangle);

        OperationResult {
            operations: self,
            results: (triangle,),
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
        triangles.extend(&self.triangles);
    }
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

pub struct OperationResult<'r, T> {
    operations: &'r mut OpsLog,
    results: T,
}

impl<'r, T> OperationResult<'r, T> {
    pub fn vertex(self, point: impl Into<Vertex>) -> OperationResult<'r, T::Out>
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

    pub fn triangle(self, triangle: Triangle) -> OperationResult<'r, T::Out>
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
