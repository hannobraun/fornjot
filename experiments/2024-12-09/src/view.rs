use std::{fmt, iter};

use crate::geometry::{HandleAny, Operation, Triangle, Vertex};

#[derive(Clone)]
pub struct OperationView {
    operation: HandleAny,
    children: Vec<Self>,
    selected: Option<usize>,
}

impl OperationView {
    pub fn new(operation: HandleAny) -> Self {
        let children = operation
            .children()
            .into_iter()
            .map(|op| Self::new(HandleAny::new(op)))
            .collect();

        Self {
            operation,
            children,
            selected: None,
        }
    }

    pub fn operations(&self) -> impl Iterator<Item = (&Self, bool, usize)> {
        self.operations_inner(true, 0)
    }

    fn operations_inner(
        &self,
        selected: bool,
        indent_level: usize,
    ) -> Box<dyn Iterator<Item = (&Self, bool, usize)> + '_> {
        let self_ = iter::once((self, selected, indent_level));

        if self.selected.is_some() {
            Box::new(self_.chain(self.children.iter().enumerate().flat_map(
                move |(i, view)| {
                    let selected = Some(i) == self.selected;
                    view.operations_inner(selected, indent_level + 1)
                },
            )))
        } else {
            Box::new(self_)
        }
    }

    pub fn select_last(&mut self) {
        self.selected = Some(self.last_index());
    }

    pub fn select_next(&mut self) {
        if let Some(selected) = self.selected {
            self.selected = Some(usize::min(selected + 1, self.last_index()));
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(selected) = self.selected {
            self.selected = Some(selected.saturating_sub(1));
        }
    }

    pub fn selected(&self) -> &Self {
        self.selected
            .and_then(|selected| self.children.get(selected))
            .map(|child| child.selected())
            .unwrap_or(self)
    }

    fn last_index(&self) -> usize {
        self.children.len().saturating_sub(1)
    }
}

impl fmt::Display for OperationView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.operation)
    }
}

impl Operation for OperationView {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        self.operation.vertices(vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        self.operation.triangles(triangles);
    }

    fn children(&self) -> Vec<HandleAny> {
        self.operation.children()
    }
}
