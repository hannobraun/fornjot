use crate::geometry::{Operation, OpsLog};

pub struct OperationView {
    ops_log: OpsLog,
    pub selected: usize,
}

impl OperationView {
    pub fn new(operation: OpsLog) -> Self {
        Self {
            ops_log: operation,
            selected: 0,
        }
    }

    pub fn operations(&self) -> Vec<Box<dyn Operation>> {
        self.ops_log.children()
    }

    pub fn select_last(&mut self) {
        self.selected = self.ops_log.operations.len().saturating_sub(1);
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
