use crate::{
    objects::{Object, Objects, WithHandle},
    storage::Handle,
};

use super::{Service, State};

impl State for Objects {
    type Command = Operation;
    type Event = ObjectToInsert;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let event = ObjectToInsert {
            object: command.object,
        };
        events.push(event);
    }

    fn evolve(&mut self, event: &Self::Event) {
        event.object.clone().insert(self);
    }
}

/// Command for `Service<Objects>`
///
/// You might prefer to use [`ServiceObjectsExt::insert`], which is a convenient
/// wrapper around `Service<Objects>::execute`.
#[derive(Clone, Debug)]
pub struct Operation {
    /// The object to insert
    pub object: Object<WithHandle>,
}

/// Event produced by `Service<Objects>`
#[derive(Clone, Debug)]
pub struct ObjectToInsert {
    /// The object to insert
    pub object: Object<WithHandle>,
}

/// Convenient API for `Service<Objects>`
pub trait ServiceObjectsExt {
    /// Insert an object
    fn insert<T>(&mut self, handle: Handle<T>, object: T)
    where
        (Handle<T>, T): Into<Object<WithHandle>>;
}

impl ServiceObjectsExt for Service<Objects> {
    fn insert<T>(&mut self, handle: Handle<T>, object: T)
    where
        (Handle<T>, T): Into<Object<WithHandle>>,
    {
        self.execute(Operation {
            object: (handle, object).into(),
        });
    }
}
