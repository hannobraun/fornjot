use crate::geometry::Operation;

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
            if selected + 1 < self.operations().len() {
                self.selected = Some(selected + 1);
            }
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
