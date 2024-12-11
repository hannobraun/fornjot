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
        self.operations_inner()
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

    pub fn selected(&self) -> Self {
        self.selected
            .and_then(|selected| self.children.get(selected).cloned())
            .unwrap_or(self.clone())
    }

    fn operations_inner(&self) -> impl Iterator<Item = (&Self, bool, usize)> {
        iter::once((self, true, 0)).chain(
            self.children
                .iter()
                .enumerate()
                .map(|(i, view)| (view, Some(i) == self.selected, 1)),
        )
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
