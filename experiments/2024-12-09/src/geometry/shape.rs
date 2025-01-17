use tuples::CombinRight;

use crate::storage::Store;

use super::{
    operation::{AnyOp, Handle},
    tri_mesh::TriMesh,
    Operation,
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

impl Operation for Shape {
    fn label(&self) -> &'static str {
        "Shape"
    }

    fn tri_mesh(&self) -> TriMesh {
        if let Some(op) = self.sequence.last() {
            op.tri_mesh()
        } else {
            TriMesh::new()
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
    fn label(&self) -> &'static str {
        self.operation.label()
    }

    fn tri_mesh(&self) -> TriMesh {
        let mesh = if let Some(op) = &self.previous {
            op.tri_mesh()
        } else {
            TriMesh::new()
        };

        mesh.merge(self.operation.tri_mesh())
    }

    fn children(&self) -> Vec<AnyOp> {
        self.operation.children()
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
    pub fn add(self, op: impl Into<T>) -> ShapeExtender<'r, NewOps::Out, T>
    where
        NewOps: CombinRight<Handle<T>>,
        T: Operation + 'static,
    {
        let op = self.store.insert(op.into());

        self.sequence.push(OperationInSequence {
            operation: op.to_any(),
            previous: self.sequence.last().map(|op| AnyOp::new(op.clone())),
        });

        ShapeExtender {
            store: self.store,
            sequence: self.sequence,
            new_ops: self.new_ops.push_right(op),
        }
    }

    pub fn get_added(self) -> NewOps {
        self.new_ops
    }
}
