//! Layer infrastructure for [`Objects`]

use crate::{
    objects::{AboutToBeStored, AnyObject, Objects},
    validate::Validation,
};

use super::{Command, Event, Layer};

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
            validation.process(event, &mut Vec::new());
        }
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

impl Command<Objects> for ObjectsCommand {
    type Event = InsertObject;

    fn decide(self, _: &Objects, events: &mut Vec<Self::Event>) {
        let ObjectsCommand::InsertObject { object } = self;
        events.push(InsertObject { object });
    }
}

/// Insert an object into the stores
///
/// Event produced by `Layer<Objects>`.
#[derive(Clone, Debug)]
pub struct InsertObject {
    /// The object to insert
    pub object: AnyObject<AboutToBeStored>,
}

impl Event<Objects> for InsertObject {
    fn evolve(&self, state: &mut Objects) {
        self.object.clone().insert(state);
    }
}
