use std::{fmt, iter};

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

    pub fn select_none(&mut self) {
        self.selected = None;
    }

    pub fn selected(&self) -> &Self {
        self.selected
            .and_then(|selected| self.children.get(selected))
            .map(|child| child.selected())
            .unwrap_or(self)
    }

    pub fn selected_mut(&mut self) -> &mut Self {
        let Some(selected) = self.selected else {
            return self;
        };

        // The way this is done, first checking for `is_none` and then
        // `unwrap`ing below, is really ugly. But the borrow checker is forcing
        // my hand.
        //
        // I've tried several variations of matching, and it can't see that in
        // the `None` case, `self` no longer needs to be borrowed, preventing me
        // from returning it.

        if self.children.get_mut(selected).is_none() {
            return self;
        };

        self.children.get_mut(selected).unwrap().selected_mut()
    }

    pub fn parent_of_selected_mut(&mut self) -> &mut Self {
        let Some(selected) = self.selected else {
            return self;
        };

        // The same comment in `selected_mut` applies here too. Plus, some ugly
        // duplication.

        if self.children.get_mut(selected).is_none() {
            return self;
        };

        if self.children.get_mut(selected).unwrap().selected.is_none() {
            self
        } else {
            self.children
                .get_mut(selected)
                .unwrap()
                .parent_of_selected_mut()
        }
    }

    fn last_index(&self) -> usize {
        self.children.len().saturating_sub(1)
    }
}

impl Object for OperationView {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.operation.display(f)
    }

    fn tri_mesh(&self) -> TriMesh {
        self.operation.tri_mesh()
    }

    fn children(&self) -> Vec<HandleAny> {
        self.operation.children()
    }
}
