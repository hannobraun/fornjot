use crate::{
    geometry::TriMesh,
    object::{HandleAny, Object},
};

#[derive(Clone)]
pub struct OperationView {
    operation: HandleAny,
    children: Vec<Self>,
    selected: Option<usize>,
}

impl OperationView {
    pub fn new(operation: HandleAny) -> Self {
        let children =
            operation.children().into_iter().map(Self::new).collect();

        Self {
            operation,
            children,
            selected: None,
        }
    }

    pub fn selected(&self) -> &Self {
        self.selected
            .and_then(|selected| self.children.get(selected))
            .map(|child| child.selected())
            .unwrap_or(self)
    }
}

impl Object for OperationView {
    fn tri_mesh(&self) -> TriMesh {
        self.operation.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        self.operation.children()
    }
}
