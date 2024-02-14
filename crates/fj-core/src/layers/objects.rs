//! Layer infrastructure for [`Objects`]

use crate::objects::{AboutToBeStored, AnyObject, Objects};

use super::State;

impl State for Objects {
    type Command = ObjectsCommand;
    type Event = ObjectsEvent;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let ObjectsCommand::InsertObject { object } = command;
        events.push(ObjectsEvent { object });
    }

    fn evolve(&mut self, event: &Self::Event) {
        event.object.clone().insert(self);
    }
}

/// Command for `Layer<Objects>`
#[derive(Debug)]
pub enum ObjectsCommand {
    /// Insert an object into the stores
    ///
    /// This is the one primitive operation that all other operations are built
    /// upon.
    InsertObject {
        /// The object to insert
        object: AnyObject<AboutToBeStored>,
    },
}

/// Event produced by `Layer<Objects>`
#[derive(Clone, Debug)]
pub struct ObjectsEvent {
    /// The object to insert
    pub object: AnyObject<AboutToBeStored>,
}
