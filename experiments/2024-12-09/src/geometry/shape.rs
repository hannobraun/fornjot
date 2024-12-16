use std::{fmt, marker::PhantomData};

use tuples::CombinRight;

use crate::storage::Store;

use super::{
    operation::{Handle, HandleAny},
    Operation, Triangle, Vertex,
};

#[derive(Default)]
pub struct Shape {
    sequence: Vec<OperationInSequence>,
}

impl Shape {
    pub fn extend_with<T>(&mut self, _: &mut Store<T>) -> ShapeExtender<(), T> {
        ShapeExtender::new(&mut self.sequence)
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

pub struct ShapeExtender<'r, NewOps, T> {
    sequence: &'r mut Vec<OperationInSequence>,
    new_ops: NewOps,

    // In principle, this isn't necessary, as `NewOps` and `T` are partially
    // redundant. The way this struct is set up right now, it's only possible to
    // add a single type of operation. So in theory, `NewOps` could be replaced
    // with `[T; N]`, and we'd just need to track the value of `N` in addition
    // to `T`.
    //
    // Unfortunately, the state of const expressions in Rust (at the time of
    // writing) doesn't allow for this.
    _t: PhantomData<T>,
}

impl<'r, T> ShapeExtender<'r, (), T> {
    fn new(sequence: &'r mut Vec<OperationInSequence>) -> Self {
        Self {
            sequence,
            new_ops: (),
            _t: PhantomData,
        }
    }
}

impl<'r, NewOps, T> ShapeExtender<'r, NewOps, T> {
    pub fn add(self, vertex: impl Into<T>) -> ShapeExtender<'r, NewOps::Out, T>
    where
        NewOps: CombinRight<Handle<T>>,
        T: Operation + 'static,
    {
        let vertex = Handle::new(vertex.into());

        self.sequence.push(OperationInSequence {
            operation: vertex.to_any(),
            previous: self.sequence.last().map(|op| HandleAny::new(op.clone())),
        });

        ShapeExtender {
            sequence: self.sequence,
            new_ops: self.new_ops.push_right(vertex),
            _t: PhantomData,
        }
    }

    pub fn get_added(self) -> NewOps {
        self.new_ops
    }
}
