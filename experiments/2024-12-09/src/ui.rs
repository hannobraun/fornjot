use std::fmt;

use crate::geometry::{Operation, Triangle, Vertex};

pub struct OperationView {
    operation: Box<dyn Operation>,
    selected: Option<usize>,
}

impl OperationView {
    pub fn new(operation: impl Operation + 'static) -> Self {
        Self {
            operation: Box::new(operation),
            selected: None,
        }
    }

    pub fn operations(&self) -> Vec<(Box<dyn Operation>, bool)> {
        self.operation
            .children()
            .into_iter()
            .enumerate()
            .map(|(i, op)| (op, Some(i) == self.selected))
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

    pub fn selected(&self) -> Option<Box<dyn Operation>> {
        let selected = self.selected?;

        self.operations()
            .into_iter()
            .nth(selected)
            .map(|(op, _)| op)
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

    fn children(&self) -> Vec<Box<dyn Operation>> {
        self.operation.children()
    }
}
