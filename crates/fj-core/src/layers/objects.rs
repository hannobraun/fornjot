//! Layer infrastructure for [`Objects`]

use crate::{
    geometry::Geometry,
    objects::{AboutToBeStored, AnyObject, Objects},
    validation::Validation,
};

use super::{validation::ValidateObject, Command, Event, Layer};

impl Layer<Objects> {
    /// Insert an object into the stores
    ///
    /// Passes any events produced to the validation layer.
    pub fn insert(
        &mut self,
        object: AnyObject<AboutToBeStored>,
        geometry: &Geometry,
        validation: &mut Layer<Validation>,
    ) {
        let mut events = Vec::new();
        self.process(InsertObject { object }, &mut events);

        for event in events {
            let event = ValidateObject {
                object: event.object.into(),
                geometry,
            };
            validation.process(event, &mut Vec::new());
        }
    }
}

/// Insert an object into the stores
#[derive(Clone, Debug)]
pub struct InsertObject {
    /// The object to insert
    pub object: AnyObject<AboutToBeStored>,
}

impl Command<Objects> for InsertObject {
    type Result = ();
    type Event = InsertObject;

    fn decide(self, _: &Objects, events: &mut Vec<Self::Event>) {
        events.push(self);
    }
}

impl Event<Objects> for InsertObject {
    fn evolve(&self, state: &mut Objects) {
        self.object.clone().insert(state);
    }
}
