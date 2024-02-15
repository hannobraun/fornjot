//! Layer infrastructure for [`Objects`]

use crate::{
    objects::{AboutToBeStored, AnyObject, Objects},
    validate::Validation,
};

use super::{Event, Layer, State};

impl Layer<Objects> {
    /// Insert and object into the stores
    pub fn insert(
        &mut self,
        object: AnyObject<AboutToBeStored>,
        validation: &mut Layer<Validation>,
    ) {
        let mut events = Vec::new();
        self.process(ObjectsCommand::InsertObject { object }, &mut events);

        for event in events {
            validation.on_objects_event(event);
        }
    }
}

impl State for Objects {
    type Command = ObjectsCommand;
    type Event = ObjectsEvent;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let ObjectsCommand::InsertObject { object } = command;
        events.push(ObjectsEvent::InsertObject { object });
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
pub enum ObjectsEvent {
    /// Insert an object into the stores
    InsertObject {
        /// The object to insert
        object: AnyObject<AboutToBeStored>,
    },
}

impl Event<Objects> for ObjectsEvent {
    fn evolve(&self, state: &mut Objects) {
        let ObjectsEvent::InsertObject { object } = self;
        object.clone().insert(state);
    }
}
