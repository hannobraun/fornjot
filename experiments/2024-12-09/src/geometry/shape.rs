use std::fmt;

use tuples::CombinRight;

use super::{
    operation::{Handle, HandleAny},
    Operation, Triangle, Vertex,
};

#[derive(Default)]
pub struct Shape {
    sequence: Vec<OperationInSequence>,
}

impl Shape {
    pub fn extend(&mut self) -> ShapeExtender<()> {
        ShapeExtender {
            sequence: &mut self.sequence,
            new_ops: (),
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
        if let Some(op) = self.sequence.last() {
            op.vertices(vertices);
        }
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        if let Some(op) = self.sequence.last() {
            op.triangles(triangles);
        }
    }

    fn children(&self) -> Vec<HandleAny> {
        self.sequence
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

pub struct ShapeExtender<'r, T> {
    sequence: &'r mut Vec<OperationInSequence>,
    new_ops: T,
}

impl<'r, T> ShapeExtender<'r, T> {
    pub fn vertex(self, vertex: impl Into<Vertex>) -> ShapeExtender<'r, T::Out>
    where
        T: CombinRight<Handle<Vertex>>,
    {
        let vertex = Handle::new(vertex.into());

        self.sequence.push(OperationInSequence {
            operation: vertex.to_any(),
            previous: self.sequence.last().map(|op| HandleAny::new(op.clone())),
        });

        ShapeExtender {
            sequence: self.sequence,
            new_ops: self.new_ops.push_right(vertex),
        }
    }

    pub fn triangle(
        self,
        triangle: impl Into<Triangle>,
    ) -> ShapeExtender<'r, T::Out>
    where
        T: CombinRight<Handle<Triangle>>,
    {
        let triangle = Handle::new(triangle.into());

        self.sequence.push(OperationInSequence {
            operation: triangle.to_any(),
            previous: self.sequence.last().map(|op| HandleAny::new(op.clone())),
        });

        ShapeExtender {
            sequence: self.sequence,
            new_ops: self.new_ops.push_right(triangle),
        }
    }

    pub fn get_new_ops(self) -> T {
        self.new_ops
    }
}
