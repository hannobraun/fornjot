use tuples::CombinRight;

use crate::storage::Store;

use super::{
    operation::{AnyOp, Handle},
    tri_mesh::TriMesh,
    Operation,
};

#[derive(Default)]
pub struct Shape {
    children: Vec<AnyOp>,
}

impl Shape {
    pub fn extend_with<'r, T>(
        &'r mut self,
        store: &'r mut Store<T>,
    ) -> ShapeExtender<'r, (), T> {
        ShapeExtender::new(store, &mut self.children)
    }
}

impl Operation for Shape {
    fn label(&self) -> &'static str {
        "Shape"
    }

    fn tri_mesh(&self) -> TriMesh {
        let mut tri_mesh = TriMesh::new();

        for op in &self.children {
            tri_mesh = tri_mesh.merge(op.tri_mesh());
        }

        tri_mesh
    }

    fn children(&self) -> Vec<AnyOp> {
        self.children
            .iter()
            .map(|op| AnyOp::new(op.clone()))
            .collect()
    }
}

pub struct ShapeExtender<'r, NewOps, T> {
    store: &'r mut Store<T>,
    ops: &'r mut Vec<AnyOp>,
    new_ops: NewOps,
}

impl<'r, T> ShapeExtender<'r, (), T> {
    fn new(store: &'r mut Store<T>, ops: &'r mut Vec<AnyOp>) -> Self {
        Self {
            store,
            ops,
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

        self.ops.push(op.to_any());

        ShapeExtender {
            store: self.store,
            ops: self.ops,
            new_ops: self.new_ops.push_right(op),
        }
    }

    pub fn get_added(self) -> NewOps {
        self.new_ops
    }
}
