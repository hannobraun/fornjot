use crate::objects::{Object, Objects, WithHandle};

use super::State;

impl State for Objects {
    type Command = Operation;
    type Event = InsertObject;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let Operation::InsertObject { object } = command;
        events.push(InsertObject { object });
    }

    fn evolve(&mut self, event: &Self::Event) {
        event.object.clone().insert(self);
    }
}

/// Command for `Service<Objects>`
#[derive(Debug)]
pub enum Operation {
    /// Insert an object into the stores
    ///
    /// This is the one primitive operation that all other operations are built
    /// upon.
    InsertObject {
        /// The object to insert
        object: Object<WithHandle>,
    },
}

/// Event produced by `Service<Objects>`
#[derive(Clone, Debug)]
pub struct InsertObject {
    /// The object to insert
    pub object: Object<WithHandle>,
}
