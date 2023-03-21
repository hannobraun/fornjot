use crate::{
    objects::{Object, Objects, WithHandle},
    storage::Handle,
};

use super::{Service, State};

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
    ///
    /// You might prefer to use [`ServiceObjectsExt::insert`], which is a
    /// convenient wrapper around `Service<Objects>::execute`.
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
        self.execute(Operation::InsertObject {
            object: (handle, object).into(),
        });
    }
}
