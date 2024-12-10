use crate::geometry::{Operation, OpsLog};

pub struct OperationView {
    ops_log: OpsLog,
    selected: usize,
}

impl OperationView {
    pub fn new(operation: OpsLog) -> Self {
        Self {
            ops_log: operation,
            selected: 0,
        }
    }

    pub fn operations(&self) -> Vec<(Box<dyn Operation>, bool)> {
        self.ops_log
            .children()
            .into_iter()
            .enumerate()
            .map(|(i, op)| (op, i == self.selected))
            .collect()
    }

    pub fn select_last(&mut self) {
        self.selected = self.operations().len().saturating_sub(1);
    }

    pub fn select_next(&mut self) {
        if self.selected < self.ops_log.operations.len() {
            self.selected += 1;
        }
    }

    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn selected(&self) -> Option<&dyn Operation> {
        self.ops_log
            .operations
            .get(self.selected)
            .map(|op| op as &_)
    }
}
