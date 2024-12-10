use crate::geometry::Operation;

pub struct OperationView {
    operation: Box<dyn Operation>,
    selected: usize,
}

impl OperationView {
    pub fn new(operation: impl Operation + 'static) -> Self {
        Self {
            operation: Box::new(operation),
            selected: 0,
        }
    }

    pub fn operations(&self) -> Vec<(Box<dyn Operation>, bool)> {
        self.operation
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
        if self.selected < self.operations().len() {
            self.selected += 1;
        }
    }

    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn selected(&self) -> Option<Box<dyn Operation>> {
        self.operations()
            .into_iter()
            .nth(self.selected)
            .map(|(op, _)| op)
    }
}
