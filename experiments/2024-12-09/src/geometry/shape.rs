use std::fmt;

use tuples::CombinRight;

use crate::{storage::Store, topology::Vertex};

use super::{
    operation::{AnyOp, Handle},
    Operation, Triangle,
};

#[derive(Default)]
pub struct Shape {
    sequence: Vec<OperationInSequence>,
}

impl Shape {
    pub fn extend_with<'r, T>(
        &'r mut self,
        store: &'r mut Store<T>,
    ) -> ShapeExtender<'r, (), T> {
        ShapeExtender::new(store, &mut self.sequence)
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

    fn children(&self) -> Vec<AnyOp> {
        self.sequence
            .iter()
            .map(|op| AnyOp::new(op.clone()))
            .collect()
    }
}

#[derive(Clone)]
struct OperationInSequence {
    pub operation: AnyOp,
    pub previous: Option<AnyOp>,
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

pub struct ShapeExtender<'r, NewOps, T> {
    store: &'r mut Store<T>,
    sequence: &'r mut Vec<OperationInSequence>,
    new_ops: NewOps,
}

impl<'r, T> ShapeExtender<'r, (), T> {
    fn new(
        store: &'r mut Store<T>,
        sequence: &'r mut Vec<OperationInSequence>,
    ) -> Self {
        Self {
            store,
            sequence,
            new_ops: (),
        }
    }
}

impl<'r, NewOps, T> ShapeExtender<'r, NewOps, T> {
    pub fn add(self, vertex: impl Into<T>) -> ShapeExtender<'r, NewOps::Out, T>
    where
        NewOps: CombinRight<Handle<T>>,
        T: Operation + 'static,
    {
        let vertex = self.store.insert(vertex.into());

        self.sequence.push(OperationInSequence {
            operation: vertex.to_any(),
            previous: self.sequence.last().map(|op| AnyOp::new(op.clone())),
        });

        ShapeExtender {
            store: self.store,
            sequence: self.sequence,
            new_ops: self.new_ops.push_right(vertex),
        }
    }

    pub fn get_added(self) -> NewOps {
        self.new_ops
    }
}
