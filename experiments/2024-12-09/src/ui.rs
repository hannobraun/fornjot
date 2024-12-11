use std::fmt;

use crate::geometry::{HandleAny, Operation, Triangle, Vertex};

pub struct OperationView {
    operation: HandleAny,
    selected: Option<usize>,
}

impl OperationView {
    pub fn new(operation: HandleAny) -> Self {
        Self {
            operation,
            selected: None,
        }
    }

    pub fn operations(&self) -> Vec<(Self, bool)> {
        self.operation
            .children()
            .into_iter()
            .enumerate()
            .map(|(i, op)| {
                (
                    OperationView {
                        operation: op,
                        selected: None,
                    },
                    Some(i) == self.selected,
                )
            })
            .collect()
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

    pub fn selected(&self) -> Option<Self> {
        self.selected.and_then(|selected| {
            self.operations()
                .into_iter()
                .nth(selected)
                .map(|(op, _)| op)
        })
    }

    fn last_index(&self) -> usize {
        self.operations().len().saturating_sub(1)
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
